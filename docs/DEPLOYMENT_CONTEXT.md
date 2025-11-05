# kiisrv Deployment Context

> **✅ UPDATE (2025):** kiisrv is now fully containerized!  
> See [CONTAINERIZED_DEPLOYMENT.md](./CONTAINERIZED_DEPLOYMENT.md) for the recommended deployment method.  
> The context below is kept for reference but containerized deployment is now preferred.

## Quick Copy-Paste Context for New Chat

```
I have a Rust project called kiisrv that is fully containerized and needs to be deployed to a Hetzner VPS running Debian 12.

PROJECT OVERVIEW:
- Keyboard firmware build server
- Rust 2021 + Axum web framework (async)
- Docker Compose for build environments
- SQLite databases for stats/config
- Listens on port 3001 by default

TECH STACK:
- Language: Rust 2021 edition
- Web framework: Axum 0.7 (async/Tokio)
- Build system: Docker Compose V2 (compose.yaml)
- Containers: Multi-stage Dockerfile, Debian 12 Bookworm Slim base
- Database: SQLite (rusqlite 0.32)
- File: Dockerfile, compose.yaml (defines 5 controller images)

RESOURCE USAGE:
- Idle: ~300MB RAM, ~10GB disk (with 2 Docker images)
- During build: +500MB-1GB RAM spike (30-60 seconds)
- Very low traffic expected

SERVER SETUP (already done via cloud-init):
- Hetzner VPS with Debian 12
- Non-root user with sudo
- SSH on port 2222 (hardened)
- ufw firewall enabled
- fail2ban configured

DEPLOYMENT GOALS:
1. Deploy kiisrv as a systemd service
2. Setup Nginx reverse proxy with SSL
3. Run alongside other low-traffic websites on same VPS
4. Minimal resource usage

CRITICAL BUILD REQUIREMENTS:
- Docker images need: gcc-arm-none-eabi, cmake, ninja, python3, pipenv
- build.sh exports CFLAGS=-fcommon (for GCC 12+ compatibility with old firmware)
- Containers compile ARM firmware and return .dfu.bin files

API ENDPOINTS:
- POST / or POST /download.php - Build firmware (accepts JSON)
- GET /versions - Available firmware versions
- GET /stats - Build statistics
- GET /layouts/:file - Layout files
- /tmp/ - Static files (build artifacts)

FILES STRUCTURE:
/opt/kiisrv/
├── Cargo.toml
├── compose.yaml
├── Dockerfile
├── src/
├── layouts/
├── build.sh
├── update_kll_cache.sh
├── tmp_builds/  (created at runtime)
└── tmp_config/  (created at runtime)

WHAT I NEED HELP WITH:
[Describe your specific deployment question here]
```

## How to Use This:

1. **Copy the entire content above** from "I have a Rust project..." to the end
2. **Paste into a new chat**
3. **Replace the last line** with your specific question
4. The AI will have full context without the modernization journey

## Examples:

**For deployment help:**
```
[paste context above]

WHAT I NEED HELP WITH:
Walk me through deploying this to my Hetzner server step-by-step, including setting up Nginx with SSL and running it alongside 2 other websites.
```

**For troubleshooting:**
```
[paste context above]

WHAT I NEED HELP WITH:
The service starts but Docker containers fail to build with error: [paste error]
```

**For optimization:**
```
[paste context above]

WHAT I NEED HELP WITH:
How can I reduce the Docker image sizes further or optimize build times?
```

---

**Containerized deployment guide available at:** [CONTAINERIZED_DEPLOYMENT.md](./CONTAINERIZED_DEPLOYMENT.md)

