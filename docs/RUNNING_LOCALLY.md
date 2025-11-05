# Running kiisrv Locally

Quick reference for running kiisrv in development mode.

## First Time Setup

```bash
# Create database files (required before first run)
touch config.db stats.db

# Build all images (includes kiisrv + all controllers)
docker compose -f compose.prod.yaml build

# Or build just what you need
docker compose -f compose.prod.yaml build kiisrv controller-057
```

**Important:** Always create `config.db` and `stats.db` as **files** before running containers, otherwise Docker will create them as directories.

## Running

### Foreground Mode (Recommended for Development)

See logs directly in your terminal:

```bash
docker compose -f compose.prod.yaml up
```

**Stop:** Press `Ctrl+C`

### Background Mode (Daemon)

Run in background:

```bash
docker compose -f compose.prod.yaml up -d
```

**View logs:**
```bash
docker compose -f compose.prod.yaml logs -f
docker compose -f compose.prod.yaml logs -f kiisrv  # Just kiisrv
```

**Stop:**
```bash
docker compose -f compose.prod.yaml down
```

## Testing

```bash
# Check stats endpoint
curl http://localhost:3001/stats

# Check versions
curl http://localhost:3001/versions

# Test build (use your configurator client or POST JSON)
```

## What Runs?

When you run `docker compose -f compose.prod.yaml up`:
- ✅ **kiisrv** container starts (web server on port 3001)
- ❌ **controller-*** containers do NOT start (they have `profile: manual`)

The controller containers are **images only**. kiisrv spawns them dynamically via Docker socket when a build request comes in.

## Troubleshooting

**Port 3001 already in use:**
```bash
# Find what's using it
lsof -i :3001

# Kill it or change port in compose.prod.yaml
```

**Can't connect to localhost:3001:**
```bash
# Check if container is running
docker ps

# Check logs for errors
docker compose -f compose.prod.yaml logs kiisrv

# Verify it's listening
docker exec kiisrv netstat -tlnp | grep 3001
```

**Docker socket permission denied:**
```bash
# Make sure kiisrv container can access Docker
docker exec kiisrv docker ps  # Should list containers
```

## Development Workflow

```bash
# 1. Make code changes
vim src/main.rs

# 2. Rebuild kiisrv image
docker compose -f compose.prod.yaml build kiisrv

# 3. Restart (if running in background)
docker compose -f compose.prod.yaml restart kiisrv

# Or stop and run in foreground to see logs
docker compose -f compose.prod.yaml down
docker compose -f compose.prod.yaml up
```

## Tip: Build Only What You Need

For faster development, build only the controllers you need:

```bash
# Build just kiisrv and latest controller
docker compose -f compose.prod.yaml build kiisrv controller-057

# Start the stack (controllers are images only, won't auto-start)
docker compose -f compose.prod.yaml up
```

