mod build;
mod kll;
mod versions;

use crate::build::*;
use crate::kll::*;

use indexmap::IndexMap;
use std::collections::hash_map::{DefaultHasher, HashMap};
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::process::Command;
use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use tokio::sync::Mutex;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use chrono::prelude::*;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use shared_child::SharedChild;

const BUILD_ROUTE: &str = "./tmp";

const LAYOUT_DIR: &str = "./layouts";
const BUILD_DIR: &str = "./tmp_builds";
const CONFIG_DIR: &str = "./tmp_config";

const STATS_DB_FILE: &str = "./stats.db";
const STATS_DB_SCHEMA: &str = include_str!("../schema/stats.sqlite");

const CONFIG_DB_FILE: &str = "./config.db";
const CONFIG_DB_SCHEMA: &str = include_str!("../schema/config.sqlite");

const CONTROLLER_GIT_URL: &str = "https://github.com/kiibohd/controller.git";
const CONTROLLER_GIT_REMOTE: &str = "controller";

#[derive(Clone, Deserialize)]
pub struct BuildRequest {
    pub config: KllConfig,
    pub env: String,
}

#[derive(Clone, Serialize)]
pub struct BuildResult {
    pub filename: String,
    pub success: bool,
}

#[derive(Clone)]
pub enum JobEntry {
    Building(Arc<SharedChild>),
    Finished(bool),
}

#[derive(Clone)]
pub struct AppState {
    job_queue: Arc<Mutex<HashMap<String, JobEntry>>>,
    stats_db: Arc<Mutex<Connection>>,
    versions: Arc<HashMap<String, VersionInfo>>,
}

#[derive(Debug)]
struct RequestLog {
    id: i32,
    uid: Option<i32>,
    ip_addr: String,
    os: String,
    web: bool,
    serial: Option<i32>,
    hash: String,
    board: String,
    variant: String,
    layers: i32,
    container: String,
    success: bool,
    request_time: DateTime<Utc>,
    build_duration: Option<i32>,
}

#[derive(Debug)]
struct VersionMap {
    name: String,
    channel: String,
    container: String,
    git_tag: String,
}

#[derive(Deserialize)]
struct LayoutParams {
    rev: Option<String>,
}

async fn get_layout(
    axum::extract::Path(file): axum::extract::Path<String>,
    Query(params): Query<LayoutParams>,
) -> Result<Response, StatusCode> {
    let rev = params.rev.unwrap_or_else(|| "HEAD".to_string());

    let path = std::path::PathBuf::from(format!("{}/{}", LAYOUT_DIR, file));
    let realfile = fs::read_link(&path).unwrap_or(std::path::PathBuf::from(&file));
    let realpath = format!("{}/{}", LAYOUT_DIR, realfile.to_str().unwrap());
    
    tracing::info!("Get layout {:?} ({})", file, rev);

    let result = Command::new("git")
        .args(&["show", &format!("{}:{}", rev, realpath)])
        .output()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let content = String::from_utf8_lossy(&result.stdout).to_string();

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        content,
    )
        .into_response())
}

async fn build_request(
    State(state): State<AppState>,
    axum::extract::ConnectInfo(addr): axum::extract::ConnectInfo<std::net::SocketAddr>,
    headers: axum::http::HeaderMap,
    Json(body): Json<BuildRequest>,
) -> Result<Response, StatusCode> {
    let ip = addr.ip();
    let user_agent = headers
        .get(header::USER_AGENT)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("")
        .to_string();

    let os = {
        let ua = user_agent.to_lowercase();
        if ua.contains("windows") {
            "Windows"
        } else if ua.contains("mac") {
            "Mac"
        } else if ua.contains("linux") || ua.contains("x11") {
            "Linux"
        } else {
            "Unknown"
        }
    }
    .to_string();

    let is_desktop_configurator = user_agent.to_lowercase().contains("electron");
    tracing::info!("IP: {:?}", ip);
    tracing::info!("OS: {:?}", os);
    tracing::info!("WEB: {:?}", !is_desktop_configurator);

    let request_time: DateTime<Utc> = Utc::now();

    let config = body.config;
    let container = match body.env.as_ref() {
        "lts" => "controller-050",
        "nightly" => "controller-057",
        "latest" | _ => "controller-057",
    }
    .to_string();

    let config_str = serde_json::to_string(&config).unwrap();

    let hash = {
        let mut hasher = DefaultHasher::new();
        container.hash(&mut hasher);
        config_str.hash(&mut hasher);
        let h = hasher.finish();
        format!("{:x}", h)
    };
    tracing::info!("Received request: {}", hash);

    let info = configure_build(&config, vec!["".to_string()]);
    let mut output_file = format!("{}-{}-{}.zip", info.name, info.layout, hash);

    let job: JobEntry = {
        let mut queue = state.job_queue.lock().await;
        let job = queue.get(&hash);

        if job.is_some() {
            tracing::info!(" > Existing task");
            job.unwrap().clone()
        } else {
            tracing::info!(" > Starting new build in container {}", container);

            let config_dir = format!("{}/{}", CONFIG_DIR, hash);
            fs::create_dir_all(&config_dir).expect("Could not create directory");

            let mut layers: Vec<String> = Vec::new();
            let files = generate_kll(&config, body.env == "lts");
            for file in files {
                let filename = format!("{}/{}", config_dir, file.name);
                fs::write(&filename, file.content).expect("Could not write kll file");
                layers.push(format!("{}", filename));
            }

            tracing::info!("{:?}", layers);
            let info = configure_build(&config, layers);
            let output_file = format!("{}-{}-{}.zip", info.name, info.layout, hash);
            tracing::info!("{:?}", info);

            let config_file = format!("{}/{}-{}.json", config_dir, info.name, info.layout);
            fs::write(&config_file, &config_str).expect("Could not write config file");

            let process = start_build(container.clone(), info, hash.clone(), output_file);
            let job = JobEntry::Building(Arc::new(process));
            queue.insert(hash.clone(), job.clone());
            job
        }
    };

    let (success, duration) = match job {
        JobEntry::Building(arc) => {
            let process = arc.clone();
            tracing::info!(" > Waiting for task to finish {}", process.id());
            let exit_status = process.wait().unwrap();
            let success: bool = exit_status.success();
            tracing::info!(" > Done");

            {
                let mut queue = state.job_queue.lock().await;
                let job = queue.get_mut(&hash).expect("Could not find job");
                *job = JobEntry::Finished(success);
            }

            let duration = Some(Utc::now().signed_duration_since(request_time));
            (success, duration)
        }
        JobEntry::Finished(success) => {
            tracing::info!(" > Job already finished {}. Updating time.", hash);
            (success, None)
        }
    };

    let build_duration = duration.map(|t| t.num_milliseconds());
    tracing::info!(
        "Started at: {:?}, Duration: {:?}",
        request_time,
        build_duration
    );

    let layers = vec![""];
    {
        let db = state.stats_db.lock().await;
        db.execute(
            "INSERT INTO Requests (ip_addr, os, web, hash, board, variant, layers, container, success, request_time, build_duration)
              VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params![
                ip.to_string(),
                os,
                !is_desktop_configurator,
                hash,
                info.name,
                info.layout,
                layers.len() as u32,
                container,
                success,
                request_time,
                build_duration,
            ],
        )
        .unwrap_or_else(|e| {
            tracing::error!("Error: Failed to insert request into stats db: {}", e);
            0
        });
    }

    if !success {
        output_file = format!("{}-{}-{}_error.zip", info.name, info.layout, hash);
    }

    let result = BuildResult {
        filename: format!("{}/{}", BUILD_ROUTE, output_file),
        success,
    };

    Ok((StatusCode::OK, Json(result)).into_response())
}

async fn stats(State(state): State<AppState>) -> Result<Response, StatusCode> {
    let db = state.stats_db.lock().await;

    let mut result = String::new();
    let mut total_layers: usize = 0;
    let mut total_buildtime = 0;
    let mut os_counts: HashMap<String, usize> = HashMap::new();
    let mut platform_counts: HashMap<String, usize> = HashMap::new();
    let mut keyboard_counts: HashMap<String, usize> = HashMap::new();
    let mut container_counts: HashMap<String, usize> = HashMap::new();
    let mut hashes: Vec<String> = Vec::new();
    let mut users: Vec<String> = Vec::new();

    let mut stmt = db.prepare("SELECT * FROM Requests").unwrap();
    let rows = stmt
        .query_map([], |row| {
            Ok(RequestLog {
                id: row.get(0)?,
                uid: row.get(1)?,
                ip_addr: row.get(2)?,
                os: row.get(3)?,
                web: row.get(4)?,
                serial: row.get(5)?,
                hash: row.get(6)?,
                board: row.get(7)?,
                variant: row.get(8)?,
                layers: row.get(9)?,
                container: row.get(10)?,
                success: row.get(11)?,
                request_time: row.get(12)?,
                build_duration: row.get(13)?,
            })
        })
        .unwrap();

    for row in rows {
        let request = row.unwrap();
        tracing::debug!("req: {:?}", request);

        *os_counts.entry(request.os).or_insert(0) += 1;

        let platform = if request.web { "Web" } else { "Desktop" }.to_string();
        *platform_counts.entry(platform).or_insert(0) += 1;

        let keyboard = format!("{}-{}", request.board, request.variant);
        *keyboard_counts.entry(keyboard).or_insert(0) += 1;

        *container_counts.entry(request.container).or_insert(0) += 1;

        total_layers += request.layers as usize;
        total_buildtime += request.build_duration.unwrap_or(0) as i32;

        hashes.push(request.hash);
        users.push(request.ip_addr);
    }

    let total_builds = hashes.len();
    hashes.sort();
    hashes.dedup();
    let unique_builds = hashes.len();

    users.sort();
    users.dedup();
    let unique_users = users.len();

    let cache_ratio = if unique_builds == 0 {
        0.
    } else {
        (total_builds as f32) / (unique_builds as f32)
    };

    let user_ratio = if unique_users == 0 {
        0.
    } else {
        (total_builds as f32) / (unique_users as f32)
    };

    let layers_ratio = if total_builds == 0 {
        0.
    } else {
        (total_layers as f32) / (total_builds as f32)
    };

    let build_time = if unique_builds == 0 {
        0
    } else {
        total_buildtime / (unique_builds as i32)
    };

    result += &format!("Builds: {} ({} unique)\n", total_builds, unique_builds);
    result += &format!("Cache ratio: {:.1}\n", cache_ratio);
    result += &format!("Avg time: {:.3} s\n\n", (build_time as f32) / 1000.0);
    result += &format!("Users: {} unique\n", unique_users);
    result += &format!("Avg builds per user: {:.1}\n", user_ratio);
    result += &format!("Average number of layers: {}\n\n", layers_ratio);
    result += &format!("OS Counts: {:#?}\n", os_counts);
    result += &format!("Platform Counts: {:#?}\n", platform_counts);
    result += &format!("Keyboard Counts: {:#?}\n", keyboard_counts);
    result += &format!("Version Counts: {:#?}\n\n", container_counts);

    Ok((StatusCode::OK, result).into_response())
}

async fn versions_request(State(state): State<AppState>) -> Result<Response, StatusCode> {
    let versions: HashMap<String, Option<ReleaseInfo>> = state
        .versions
        .iter()
        .map(|(k, v)| (k.clone(), v.info.clone()))
        .collect();

    Ok((StatusCode::OK, Json(versions)).into_response())
}

fn version_map(db: Connection) -> HashMap<String, VersionInfo> {
    let mut stmt = db.prepare("SELECT * FROM Versions").unwrap();
    let rows = stmt
        .query_map([], |row| {
            Ok(VersionMap {
                name: row.get(0)?,
                channel: row.get(1)?,
                container: row.get(2)?,
                git_tag: row.get(3)?,
            })
        })
        .unwrap();
    let versions: Vec<VersionMap> = rows.map(|r| r.unwrap()).collect();

    let containers = list_containers();
    let tags = fetch_tags();
    versions
        .into_iter()
        .filter(|v| containers.contains(&v.container))
        .map(|v| {
            (
                v.name,
                VersionInfo {
                    container: v.container,
                    channel: v.channel,
                    info: tags.get(&v.git_tag).cloned(),
                },
            )
        })
        .collect()
}

fn fetch_tags() -> IndexMap<String, ReleaseInfo> {
    let result = Command::new("git")
        .args(&["ls-remote", "--tags", CONTROLLER_GIT_REMOTE])
        .output()
        .expect("Failed!");
    let out = String::from_utf8_lossy(&result.stdout);
    let map = out
        .lines()
        .filter(|l| !l.contains("^{}"))
        .filter_map(|l| {
            let mut parts = l.split('\t');
            Some((parts.next()?.trim(), parts.next()?.trim()))
        });

    let mut versions = IndexMap::new();

    for (h, t) in map.rev() {
        let hash = h.to_string();
        let tag = t.replace("refs/tags/", "");

        let result = Command::new("git")
            .args(&["rev-list", "--count", h])
            .output()
            .expect("Failed!");
        let commit: u16 = String::from_utf8_lossy(&result.stdout)
            .trim()
            .parse()
            .unwrap();
        let msb = ((commit & 0xFF00) >> 8) as u8;
        let lsb = ((commit & 0x00FF) >> 0) as u8;

        fn bcd_format(x: u8) -> String {
            if x > 99 {
                format!("{:x?}", x)
            } else {
                x.to_string()
            }
        }
        let bcd = format!("{}.{}", bcd_format(msb), bcd_format(lsb));

        let result = Command::new("git")
            .args(&["log", "-1", "--pretty=tformat:%ai", h])
            .output()
            .expect("Failed!");
        let out = String::from_utf8_lossy(&result.stdout);
        let date = out.trim().to_string();

        let notes = format!("https://github.com/kiibohd/controller/releases/tag/{}", tag);
        versions.insert(
            tag,
            ReleaseInfo {
                commit,
                date,
                hash,
                notes,
                bcd,
            },
        );
    }

    versions
}

#[derive(Debug, Clone)]
pub struct VersionInfo {
    container: String,
    channel: String,
    info: Option<ReleaseInfo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReleaseInfo {
    commit: u16,
    date: String,
    hash: String,
    bcd: String,
    notes: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Check if remote exists, add it if it doesn't
    let remote_exists = Command::new("git")
        .args(&["remote", "get-url", CONTROLLER_GIT_REMOTE])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !remote_exists {
        let _ = Command::new("git")
            .args(&["remote", "add", CONTROLLER_GIT_REMOTE, CONTROLLER_GIT_URL])
            .output();
    }

    let _ = Command::new("git")
        .args(&["fetch", CONTROLLER_GIT_REMOTE])
        .output()
        .expect("Failed to fetch git remote");

    let queue: HashMap<String, JobEntry> = HashMap::new();

    let config_db = Connection::open(Path::new(CONFIG_DB_FILE)).unwrap();
    config_db.execute(CONFIG_DB_SCHEMA, []).unwrap();

    let stats_db = Connection::open(Path::new(STATS_DB_FILE)).unwrap();
    stats_db.execute(STATS_DB_SCHEMA, []).unwrap();

    let containers = list_containers();
    tracing::info!("\nPossible containers:");
    tracing::info!("{:#?}", containers);

    let versions = version_map(config_db);
    tracing::info!("\nVersions:");
    for (v, i) in versions.iter() {
        tracing::info!("{} -> {} [{}]", v, i.container, i.channel);
    }

    let state = AppState {
        job_queue: Arc::new(Mutex::new(queue)),
        stats_db: Arc::new(Mutex::new(stats_db)),
        versions: Arc::new(versions),
    };

    let app = Router::new()
        .route("/versions", get(versions_request))
        .route("/stats", get(stats))
        .route("/layouts/:file", get(get_layout))
        .nest_service("/tmp", ServeDir::new(BUILD_DIR))
        .fallback(post(build_request)) // Catch-all POST handler (like Iron's mount at "/")
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let host = std::env::var("KIISRV_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("KIISRV_PORT").unwrap_or_else(|_| "3001".to_string());
    let api_host = format!("{}:{}", host, port);

    tracing::info!("\nBuild dispatcher starting.\nListening on {}", api_host);

    let listener = tokio::net::TcpListener::bind(&api_host)
        .await
        .expect("Failed to bind to address");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await
    .unwrap();
}
