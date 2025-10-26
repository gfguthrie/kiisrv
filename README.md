# KiiSrv

Build backend for the keyboard firmware configurator.

Modernized in 2025 from 2018 codebase. See [docs/MODERNIZATION.md](docs/MODERNIZATION.md) for details.

## Quick Start

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable)
- [Docker](https://docs.docker.com/get-docker/) with Compose V2 (included with Docker Desktop)
- 8GB+ RAM recommended for Docker builds

**Note:** This project uses `docker compose` (V2), not the deprecated `docker-compose` (V1).

### Setup

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

### Running

Start the build server:
```bash
cargo run
```

The server will listen on `http://0.0.0.0:3001` (configurable via `KIISRV_HOST` and `KIISRV_PORT`).

### Testing

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

**[docs/MODERNIZATION.md](docs/MODERNIZATION.md)** - Start here! Complete guide covering:
- What changed in the 2025 modernization
- Rust framework migration (Iron → Axum)
- Docker architecture and build system
- GCC 12+ compatibility requirements
- Known issues and test results

**[docs/GCC_COMPATIBILITY.md](docs/GCC_COMPATIBILITY.md)** - Technical deep-dive on the `-fcommon` flag fix for modern GCC.

## Project Structure

```
kiisrv/
├── src/                  # Rust source (Axum server, build orchestration, KLL generation)
├── tests/                # Integration tests (100% passing)
├── layouts/              # Keyboard layout definitions (JSON)
├── docs/                 # Documentation
│   ├── MODERNIZATION.md   # Complete guide (start here!)
│   └── GCC_COMPATIBILITY.md
├── Dockerfile            # Multi-stage build (Debian slim)
├── compose.yaml          # Docker Compose V2 configuration
├── Cargo.toml           # Rust dependencies
└── README.md            # This file
```
