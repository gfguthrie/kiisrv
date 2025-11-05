# Containerization Summary

## What Changed

kiisrv is now **fully containerized** - both the Rust web server and the firmware build environments run in Docker containers.

### Before (Direct Deployment)
```
Host Machine
├── Rust/Cargo (required)
├── kiisrv binary (cargo build --release)
└── Docker containers (controller-050, controller-057, etc.)
```

**Issues:**
- ❌ Required Rust installation on server
- ❌ IPv6-only servers couldn't clone from GitHub
- ❌ ~10 minute cargo build on server
- ❌ Harder for end users to self-host

### After (Containerized)
```
Host Machine
└── Docker only
    ├── kiisrv container (Rust server)
    └── Controller containers (firmware builds)
```

**Benefits:**
- ✅ Only Docker required on server
- ✅ Build images locally, deploy anywhere
- ✅ No GitHub access needed on server
- ✅ Self-hosting: just `docker compose up`
- ✅ Reproducible builds
- ✅ Easy updates (transfer new images)

## Files Created

### 1. `.dockerignore`
Optimizes Docker build context by excluding unnecessary files.

### 2. `Dockerfile` (updated)
Added two new stages:
- **`builder`**: Compiles Rust binary in isolated environment
- **`kiisrv`**: Minimal runtime image (~150MB) with:
  - kiisrv binary
  - Docker client (for spawning controllers)
  - Non-root user for security
  - Exposed port 3001

### 3. `compose.prod.yaml`
Full production stack:
- kiisrv service (web server)
- Controller services (firmware builders)
- Docker socket mounting
- Volume management
- Network isolation

### 4. `docs/CONTAINERIZED_DEPLOYMENT.md`
Comprehensive guide covering:
- Building images locally
- Transferring to IPv6-only servers
- Production deployment
- Nginx setup
- Monitoring and maintenance
- Troubleshooting

### 5. `QUICK_START.md`
Choose-your-own-adventure guide for different use cases.

## Technical Implementation Details

### Critical Fixes for Containerization

**1. Database Initialization (`src/main.rs`)**
- Changed from `.execute()` to `.execute_batch()`
- Reason: Schema has multiple SQL statements (CREATE + 8 INSERTs)
- `.execute()` only runs first statement, `.execute_batch()` runs all
- Without this: Versions table created but empty → `/versions` returns `{}`

**2. Container Spawning (`src/build.rs`)**
- Changed from `docker compose run` to `docker run`
- Added environment variables `HOST_TMP_CONFIG` and `HOST_TMP_BUILDS`
- Reason: No compose.yaml available inside kiisrv container
- Volume mounts need HOST paths when spawning via Docker socket

**3. Container Discovery (`src/build.rs`)**
- Changed from `docker compose config --services` to `docker images`
- Reason: compose.yaml not available in container
- Now lists images directly via Docker API

**4. Layout File Serving (`src/main.rs`)**
- Changed from `git show HEAD:path` to `fs::read_to_string()`
- Reason: Layouts are regular files copied into container, not in git
- Simpler and faster than git operations

**5. Docker Socket Permissions (`Dockerfile`)**
- Added kiisrv user to group 991 (matches Mac Docker Desktop)
- Reason: Docker socket owned by root:991, user needs group access
- Also added to standard docker group for Linux compatibility

**6. Git Repository Initialization (`src/main.rs`)**
- Added `git init` before remote operations
- Reason: Container has no `.git` directory initially
- Needed for `fetch_tags()` to work correctly

**7. Runtime Dependencies (`Dockerfile`)**
- Added `git` to kiisrv runtime stage
- Reason: Application uses git commands for fetching controller tags

**8. Schema Idempotency (`schema/config.sqlite`)**
- Changed `INSERT` to `INSERT OR REPLACE`
- Reason: Allows clean restarts without database errors

## Updated Documentation

### `README.md`
- Added containerized quick start as primary method
- Kept local development as secondary option
- Added deployment comparison table

### `DEPLOYMENT_CONTEXT.md`
- Updated for containerization
- Points to new containerized deployment guide

### `docs/README.md`
- Updated to prioritize containerized deployment
- Added IMPLEMENTATION_NOTES.md reference

## Architecture

### Docker-in-Docker Pattern

The kiisrv container uses the **Docker socket mounting** pattern:

```yaml
volumes:
  - /var/run/docker.sock:/var/run/docker.sock
```

This allows kiisrv to spawn controller containers as **siblings** (not children):

```
┌─────────────────────────────────────────┐
│  Docker Host                            │
│                                         │
│  ┌─────────────────┐                   │
│  │ kiisrv          │                   │
│  │ (web server)    │──spawns──┐        │
│  └─────────────────┘          │        │
│                                ▼        │
│                    ┌──────────────────┐ │
│                    │ controller-057   │ │
│                    │ (firmware build) │ │
│                    └──────────────────┘ │
└─────────────────────────────────────────┘
```

### Multi-Stage Dockerfile

```
base (Debian slim)
├── [build tools, ARM compiler, Python, KLL]
│
├── controller (firmware builders)
│   └── [clones controller repo, caches KLL]
│
└── builder (Rust compiler)
    └── kiisrv (runtime)
        └── [binary, Docker client, layouts]
```

Benefits:
- Shared base layer (caching)
- Minimal final image
- No build artifacts in runtime

## Deployment Workflows

### Workflow 1: IPv6-only Server (Recommended)

```bash
# Local machine
docker compose -f compose.prod.yaml build
docker save kiisrv-server:latest | gzip > kiisrv-server.tar.gz
scp kiisrv-server.tar.gz server:/tmp/

# Server
docker load < /tmp/kiisrv-server.tar.gz
docker compose -f compose.prod.yaml up -d
```

### Workflow 2: Direct Build (IPv4 available)

```bash
# Server
git clone https://github.com/kiibohd/kiisrv.git
cd kiisrv
docker compose -f compose.prod.yaml build
docker compose -f compose.prod.yaml up -d
```

### Workflow 3: Self-Hosting

```bash
# Any machine with Docker
docker compose -f compose.prod.yaml up -d
# Done! http://localhost:3001
```

## Build Times

| Stage | Time | Notes |
|-------|------|-------|
| **Base layer** | 3-5 min | Cached after first build |
| **Controller-057** | 10-15 min | Per controller version |
| **Rust builder** | 5-10 min | Cached after first build |
| **kiisrv runtime** | 1-2 min | Fast (just copies files) |
| **Total (first build)** | 20-30 min | All images |
| **Total (incremental)** | 1-5 min | Only changed layers |

## Image Sizes

| Image | Size | Contents |
|-------|------|----------|
| **kiisrv-server** | ~150MB | Binary + Docker client |
| **controller-050** | ~2.5GB | Full build environment |
| **controller-057** | ~2.5GB | Full build environment |

## Security

1. **Non-root user**: kiisrv runs as uid 1000
2. **Docker socket**: Required for spawning containers (effectively root)
3. **Network isolation**: kiisrv network separates services
4. **Minimal base**: Debian slim reduces attack surface
5. **No secrets in images**: API key mounted at runtime

## Compatibility

- ✅ Works with existing configurator clients (API unchanged)
- ✅ Compatible with existing layout files
- ✅ Same database schemas
- ✅ Backward compatible with `compose.yaml` (controller-only builds)

## Testing Checklist

- [ ] Build all images locally
- [ ] Start stack with `compose.prod.yaml`
- [ ] Test firmware build (POST /)
- [ ] Verify stats endpoint (GET /stats)
- [ ] Check versions endpoint (GET /versions)
- [ ] Test layout files (GET /layouts/*)
- [ ] Verify build artifacts in tmp_builds/
- [ ] Check logs (`docker compose logs`)
- [ ] Test on IPv6-only server (image transfer)
- [ ] Verify Nginx reverse proxy
- [ ] Test SSL certificate

## Migration Guide (Existing Deployments)

If you have kiisrv running directly (non-containerized):

```bash
# 1. Build containerized version locally
docker compose -f compose.prod.yaml build

# 2. Stop existing service
sudo systemctl stop kiisrv

# 3. Backup databases
cp /opt/kiisrv/*.db /opt/kiisrv/backup/

# 4. Start containerized version
cd /opt/kiisrv
docker compose -f compose.prod.yaml up -d

# 5. Update Nginx (no changes needed, still proxies :3001)

# 6. Test
curl http://localhost:3001/stats

# 7. Remove old systemd service (optional)
sudo systemctl disable kiisrv
sudo rm /etc/systemd/system/kiisrv.service
```

## Future Enhancements

1. **Multi-arch builds**: Support ARM64 for Raspberry Pi, Apple Silicon
2. **Image registry**: Publish to Docker Hub for easier distribution
3. **GitHub Actions**: Automated builds and testing
4. **Health checks**: Add Docker health checks to compose file
5. **Resource limits**: Add memory/CPU limits in production compose
6. **Versioning**: Tag images with git tags for rollback
7. **Monitoring**: Prometheus metrics export

## Resources

- **Containerized Deployment Guide**: [CONTAINERIZED_DEPLOYMENT.md](./CONTAINERIZED_DEPLOYMENT.md)
- **Implementation Details**: [IMPLEMENTATION_NOTES.md](./IMPLEMENTATION_NOTES.md)
- **Quick Start**: [../QUICK_START.md](../QUICK_START.md)
- **README**: [../README.md](../README.md)
- **Documentation Index**: [../DOCUMENTATION_INDEX.md](../DOCUMENTATION_INDEX.md)

## Summary

kiisrv is now a **fully containerized application** that:
- ✅ Solves IPv6-only server deployment issues
- ✅ Makes self-hosting trivial for end users
- ✅ Provides reproducible builds
- ✅ Simplifies server setup (Docker only)
- ✅ Maintains full backward compatibility

The containerized approach is now the **recommended deployment method** for all use cases.

