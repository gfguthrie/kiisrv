# KiiSrv

[![Docker Build](https://github.com/kiibohd/kiisrv/actions/workflows/docker-build-publish.yml/badge.svg)](https://github.com/kiibohd/kiisrv/actions/workflows/docker-build-publish.yml)

Build backend for the keyboard firmware configurator.

Modernized in 2025 from 2018 codebase. See [docs/MODERNIZATION.md](docs/MODERNIZATION.md) for details.

## Quick Start

### Containerized Deployment (Recommended)

**For production deployment or self-hosting:**

#### Option A: Using Pre-Built Images (Fastest - No Build Required!)

```bash
# 1. Install Docker
curl -fsSL https://get.docker.com | sudo sh

# 2. Clone or download this repository
git clone https://github.com/kiibohd/kiisrv.git
cd kiisrv

# 3. Create database files (required before first start)
touch config.db stats.db
chmod 666 config.db stats.db  # Allow container to write

# 4. Optional: Add GitHub token to avoid rate limits
echo "your_github_token" > apikey

# 5. Pull pre-built images and start (2-3 minutes)
docker compose -f compose.ghcr.yaml pull
docker compose -f compose.ghcr.yaml up -d

# 6. Verify it's running
curl http://localhost:3001/versions
curl http://localhost:3001/stats
```

**See [docs/GITHUB_ACTIONS_DEPLOYMENT.md](docs/GITHUB_ACTIONS_DEPLOYMENT.md) for pre-built image details.**

#### Option B: Build Locally

```bash
# 1-4. Same as above

# 5. Build and start (20-30 minutes first time)
docker compose -f compose.prod.yaml build
docker compose -f compose.prod.yaml up -d

# 6. Verify (same as above)
```

**Important:** Create empty `config.db` and `stats.db` files before running `docker compose up`, otherwise Docker will create them as directories and the server will fail to start.

**See [docs/CONTAINERIZED_DEPLOYMENT.md](docs/CONTAINERIZED_DEPLOYMENT.md) for:**
- Deploying to IPv6-only servers (build locally, transfer images)
- Production setup with Nginx and SSL
- Management and monitoring commands
- Troubleshooting

### Local Development

**For development on the codebase itself:**

#### Prerequisites
- [Rust](https://rustup.rs/) (latest stable)
- [Docker](https://docs.docker.com/get-docker/) with Compose V2 (included with Docker Desktop)
- 8GB+ RAM recommended for Docker builds

**Note:** This project uses `docker compose` (V2), not the deprecated `docker-compose` (V1).

#### Setup

1. **Install Rust:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Build a controller image** (just one to start):
   ```bash
   docker compose build controller-057
   ```

   Or build all (requires significant resources):
   ```bash
   docker compose build
   ```

3. **Optional:** Add a [GitHub access token](https://github.com/settings/tokens) to the `apikey` file to prevent rate limiting.

#### Running

Start the build server:
```bash
cargo run
```

The server will listen on `http://0.0.0.0:3001` (configurable via `KIISRV_HOST` and `KIISRV_PORT`).

#### Testing

Run all tests (41 integration tests):
```bash
cargo test
```

All tests should pass (100% pass rate).

## API Endpoints

The server provides the following endpoints:

- `POST /` or `POST /download.php` - Build firmware (accepts JSON config)
- `GET /versions` - Available firmware versions
- `GET /stats` - Build statistics  
- `GET /layouts/:file` - Keyboard layout files
- `/tmp/` - Static file serving for build artifacts

## Architecture

- **Rust Server** (`cargo run`): Receives build requests, orchestrates Docker containers
- **Docker Containers**: Pre-configured build environments for different firmware versions
  - `controller-050` - LTS version (v0.5.0)
  - `controller-057` - Latest version (v0.5.7)
  - Others: v0.5.4, v0.5.5, v0.5.6

See [docs/MODERNIZATION.md](docs/MODERNIZATION.md) for architecture details.

## Debugging

**Get a shell in a container:**
```bash
docker compose run --entrypoint /bin/bash controller-057
```

**Check build logs:**
```bash
ls -la tmp_builds/
# Build logs are in: tmp_builds/<build-id>/log/build.log
```

**View server logs:**
```bash
# Server logs go to stdout when running `cargo run`
# For production, redirect to a log file
```

## Adding New Firmware Versions

1. **Add service to compose.yaml:**
   ```yaml
   controller-058:
     << : *controller-template
     build:
       context: .
       dockerfile: Dockerfile
       target: controller
       args:
         - TAG=v0.5.8
     image: kiisrv-controller-058:latest
   ```

2. **Build the container:**
   ```bash
   docker compose build controller-058
   ```

3. **Update version mapping in src/main.rs** (if needed):
   ```rust
   let container = match body.env.as_ref() {
       "lts" => "controller-050",
       "latest" | _ => "controller-058",  // Update this
   }
   ```

4. **Restart the server**

## Documentation

**[docs/GITHUB_ACTIONS_DEPLOYMENT.md](docs/GITHUB_ACTIONS_DEPLOYMENT.md)** - Using pre-built Docker images:
- Deploy without building (2-3 min vs 20-30 min)
- Self-hosting with pre-built images
- GitHub Container Registry integration
- Automated builds and publishing

**[docs/CONTAINERIZED_DEPLOYMENT.md](docs/CONTAINERIZED_DEPLOYMENT.md)** - Containerized deployment guide:
- Production deployment (build locally, deploy anywhere)
- Self-hosting instructions
- IPv6-only server solutions
- Nginx setup and SSL configuration
- Troubleshooting common issues

**[docs/IMPLEMENTATION_NOTES.md](docs/IMPLEMENTATION_NOTES.md)** - Technical implementation details:
- Code changes required for containerization
- Docker-in-Docker pattern explanation
- Challenge-solution breakdown
- Backward compatibility notes

**[docs/MODERNIZATION.md](docs/MODERNIZATION.md)** - Modernization overview:
- What changed in the 2025 modernization
- Rust framework migration (Iron → Axum)
- Docker architecture and build system
- GCC 12+ compatibility requirements
- Known issues and test results

**[docs/GCC_COMPATIBILITY.md](docs/GCC_COMPATIBILITY.md)** - Technical deep-dive on the `-fcommon` flag fix for modern GCC.

## Deployment Options

| Method | Best For | Pros | Cons |
|--------|----------|------|------|
| **Containerized** | Production, self-hosting, IPv6-only servers | Easy setup, portable, reproducible | Requires Docker |
| **Direct** | Development, modification | Direct access to code | Requires Rust, manual setup |

## Project Structure

```
kiisrv/
├── src/                        # Rust source (Axum server, build orchestration, KLL generation)
├── tests/                      # Integration tests (100% passing)
├── layouts/                    # Keyboard layout definitions (JSON)
├── docs/                       # Documentation (11 guides)
│   ├── GITHUB_ACTIONS_DEPLOYMENT.md  # Pre-built images guide ⭐
│   ├── GITHUB_ACTIONS_SETUP.md       # CI/CD setup instructions
│   ├── CONTAINERIZED_DEPLOYMENT.md   # Full deployment guide
│   ├── DEPLOYMENT_CHECKLIST.md       # Production deployment steps
│   ├── IMPLEMENTATION_NOTES.md       # Technical implementation details
│   ├── RUNNING_LOCALLY.md            # Local development guide
│   ├── CONTAINERIZATION_SUMMARY.md   # Containerization overview
│   ├── MODERNIZATION.md              # Complete modernization guide
│   └── ... (3 more)
├── .github/workflows/          # GitHub Actions CI/CD
│   └── docker-build-publish.yml      # Automated image builds
├── Dockerfile                  # Multi-stage build (kiisrv + controllers)
├── compose.yaml                # Docker Compose for controller builds (cargo run)
├── compose.prod.yaml           # Full containerized stack (build locally)
├── compose.ghcr.yaml           # Use pre-built images (fastest) ⭐
├── README.md                   # This file (start here!)
├── QUICK_START.md              # Fast deployment commands
├── DOCUMENTATION_INDEX.md      # Complete docs index
└── Cargo.toml                  # Rust dependencies
```

**📚 See [DOCUMENTATION_INDEX.md](DOCUMENTATION_INDEX.md) for complete documentation guide.**
