# Containerized Deployment Guide

This guide covers deploying kiisrv as a fully containerized application, including the Rust web server itself (not just the firmware build environments).

## Why Containerize?

1. **Solves IPv6-only VPS issues**: Build Docker images locally, then deploy to servers without GitHub access
2. **Easier self-hosting**: Users can deploy with just `docker compose up`
3. **Consistent environments**: Same setup works on any Linux server
4. **Portable**: Easy to move between servers or cloud providers

## Architecture

```
┌─────────────────────────────────────────┐
│  Docker Host (Hetzner VPS, local, etc)  │
│                                         │
│  ┌─────────────────────────────────┐   │
│  │  kiisrv Container               │   │
│  │  - Rust web server (Axum)       │   │
│  │  - Exposes port 3001            │   │
│  │  - Mounts Docker socket         │   │
│  └───────────┬─────────────────────┘   │
│              │ spawns                  │
│              ▼                          │
│  ┌─────────────────────────────────┐   │
│  │  Controller Containers          │   │
│  │  - controller-050 (v0.5.0 LTS)  │   │
│  │  - controller-057 (v0.5.7)      │   │
│  │  - controller-05x (others)      │   │
│  └─────────────────────────────────┘   │
│                                         │
└─────────────────────────────────────────┘
```

The kiisrv container has access to the host's Docker socket, allowing it to spawn controller containers as siblings (not children).

## Deployment Options

### Option 1: Build Locally, Deploy Anywhere (Recommended for IPv6-only servers)

**When to use:**
- Your VPS can't access GitHub (IPv6-only, firewall, etc.)
- You want reproducible builds
- You want to test locally before deploying

**Steps:**

```bash
# 1. Build all images locally (on your Mac/workstation)
cd ~/Developer/forks/kiisrv

# Build the kiisrv server image
docker compose -f compose.prod.yaml build kiisrv

# Build controller images (choose which versions you need)
docker compose -f compose.prod.yaml build controller-050  # LTS
docker compose -f compose.prod.yaml build controller-057  # Latest

# 2. Save images to tar files
docker save kiisrv-server:latest | gzip > kiisrv-server.tar.gz
docker save kiisrv-controller-050:latest | gzip > controller-050.tar.gz
docker save kiisrv-controller-057:latest | gzip > controller-057.tar.gz

# 3. Copy to server
scp -P 2222 kiisrv-server.tar.gz youruser@your-server:/tmp/
scp -P 2222 controller-050.tar.gz youruser@your-server:/tmp/
scp -P 2222 controller-057.tar.gz youruser@your-server:/tmp/

# 4. Copy compose file and data
rsync -avz -e "ssh -p 2222" \
  --exclude target \
  --exclude tmp_builds \
  --exclude tmp_config \
  --exclude .git \
  compose.prod.yaml \
  layouts/ \
  schema/ \
  youruser@your-server:/opt/kiisrv/

# 5. On the server: Load images
ssh -p 2222 youruser@your-server
cd /opt/kiisrv
docker load < /tmp/kiisrv-server.tar.gz
docker load < /tmp/controller-050.tar.gz
docker load < /tmp/controller-057.tar.gz

# 6. Create required files/directories
mkdir -p tmp_builds tmp_config

# IMPORTANT: Create database files BEFORE starting containers
# If these don't exist, Docker will create them as directories instead
touch config.db stats.db

echo "your_github_token_here" > apikey  # Optional

# 7. Start the stack
docker compose -f compose.prod.yaml up -d

# 8. Verify
docker compose -f compose.prod.yaml ps
curl http://localhost:3001/stats
```

### Option 2: Build on Server

**When to use:**
- Your server has IPv4 connectivity and can access GitHub
- You prefer building directly on the deployment server

```bash
# On the server
cd /opt/kiisrv

# Build images
docker compose -f compose.prod.yaml build

# Start the stack
docker compose -f compose.prod.yaml up -d
```

## Server Setup (Hetzner or Any VPS)

### Prerequisites

```bash
# Install Docker (Debian 12 / Ubuntu 22.04+)
curl -fsSL https://get.docker.com | sudo sh

# Add your user to docker group
sudo usermod -aG docker $USER

# Log out and back in
exit
# reconnect...

# Verify
docker --version
docker compose version
```

### Deploy kiisrv

```bash
# Create deployment directory
sudo mkdir -p /opt/kiisrv
sudo chown $USER:$USER /opt/kiisrv
cd /opt/kiisrv

# Get the compose file (copy from local or git clone if you have IPv4)
# See Option 1 above for copying from local

# Create runtime directories
mkdir -p tmp_builds tmp_config

# CRITICAL: Create database files BEFORE starting
# If they don't exist, Docker will create directories instead of files
touch config.db stats.db

# Optional: Add GitHub API key
echo "your_github_token" > apikey

# Start the stack
docker compose -f compose.prod.yaml up -d

# View logs
docker compose -f compose.prod.yaml logs -f kiisrv
```

### Setup Nginx Reverse Proxy

```bash
# Install Nginx
sudo apt install -y nginx certbot python3-certbot-nginx

# Create Nginx config
sudo tee /etc/nginx/sites-available/kiisrv << 'EOF'
server {
    listen 80;
    server_name configurator.yourdomain.com;

    # Long timeouts for firmware compilation
    proxy_read_timeout 600s;
    proxy_connect_timeout 600s;
    proxy_send_timeout 600s;

    # Large uploads for keyboard configs
    client_max_body_size 10M;

    location / {
        proxy_pass http://127.0.0.1:3001;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
EOF

# Enable site
sudo ln -s /etc/nginx/sites-available/kiisrv /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx

# Setup SSL
sudo certbot --nginx -d configurator.yourdomain.com
```

### Firewall Configuration

```bash
# Allow HTTP/HTTPS
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw enable
sudo ufw status
```

## Management Commands

### Start/Stop/Restart

```bash
cd /opt/kiisrv

# Start
docker compose -f compose.prod.yaml up -d

# Stop
docker compose -f compose.prod.yaml down

# Restart
docker compose -f compose.prod.yaml restart

# Restart just kiisrv (not controllers)
docker compose -f compose.prod.yaml restart kiisrv
```

### View Logs

```bash
# All services
docker compose -f compose.prod.yaml logs -f

# Just kiisrv
docker compose -f compose.prod.yaml logs -f kiisrv

# Last 100 lines
docker compose -f compose.prod.yaml logs --tail=100 kiisrv
```

### Updates

```bash
# 1. Build new images locally (on your Mac)
cd ~/Developer/forks/kiisrv
git pull origin modernize
docker compose -f compose.prod.yaml build kiisrv

# 2. Save and transfer
docker save kiisrv-server:latest | gzip > kiisrv-server.tar.gz
scp -P 2222 kiisrv-server.tar.gz youruser@your-server:/tmp/

# 3. On server: Load and restart
ssh -p 2222 youruser@your-server
docker load < /tmp/kiisrv-server.tar.gz
cd /opt/kiisrv
docker compose -f compose.prod.yaml up -d
```

### Cleanup

```bash
# Remove old build artifacts (older than 7 days)
find /opt/kiisrv/tmp_builds -mtime +7 -type f -delete

# Clean up Docker
docker system prune -f

# Clean up old images
docker image prune -a -f
```

## Monitoring

### Health Checks

```bash
# Check if containers are running
docker compose -f compose.prod.yaml ps

# Check kiisrv health
curl http://localhost:3001/stats
curl http://localhost:3001/versions

# Check resource usage
docker stats --no-stream
```

### Resource Usage

**Typical usage:**
- **kiisrv container**: 20-50MB RAM (idle)
- **Controller containers**: Only run during builds (200-500MB during build)
- **Disk**: ~10-15GB for 2-3 controller images
- **During build**: +500MB-1GB RAM spike (30-60 seconds)

## Self-Hosting Instructions (For End Users)

If you want to self-host kiisrv:

```bash
# 1. Install Docker
curl -fsSL https://get.docker.com | sudo sh

# 2. Clone the repository (or download release)
git clone https://github.com/kiibohd/kiisrv.git
cd kiisrv

# 3. Build images (this takes 15-30 minutes)
docker compose -f compose.prod.yaml build

# 4. Start the server
docker compose -f compose.prod.yaml up -d

# 5. Access at http://localhost:3001
curl http://localhost:3001/stats
```

That's it! Point your configurator to `http://localhost:3001`.

## Troubleshooting

### kiisrv container won't start

```bash
# Check logs
docker compose -f compose.prod.yaml logs kiisrv

# Common issues:
# - Port 3001 already in use: Change KIISRV_PORT in compose.prod.yaml
# - Database permission errors: Check ownership of *.db files
# - Docker socket permission denied: User needs to be in group matching socket GID
```

### Empty versions endpoint returns {}

```bash
# Check if databases exist
docker exec kiisrv ls -la /app/*.db

# Check if images are visible
docker exec kiisrv docker images | grep kiisrv-controller

# If permission denied on docker command:
# - Rebuild kiisrv image (includes group 991 fix for Mac)
# - On Linux servers, socket GID may differ - adjust Dockerfile if needed
```

### Firmware builds fail with "unknown flag" error

```bash
# Check logs for the exact error
docker logs kiisrv 2>&1 | grep "unknown"

# This was fixed by switching from 'docker compose run' to 'docker run'
# Ensure you're using the latest kiisrv-server:latest image
docker compose -f compose.prod.yaml build kiisrv
```

### Database files become directories

```bash
# This happens if files don't exist before 'docker compose up'
# Fix:
cd /opt/kiisrv
docker compose -f compose.prod.yaml down
rm -rf config.db stats.db  # Remove directories
touch config.db stats.db   # Create empty files
docker compose -f compose.prod.yaml up -d
```

### Controller containers not found

```bash
# List available images
docker images | grep kiisrv

# Build missing controller
docker compose -f compose.prod.yaml build controller-057
```

### Builds failing

```bash
# Check controller container logs
docker logs kiisrv-controller-057

# Check kiisrv build directory
ls -la /opt/kiisrv/tmp_builds/

# Verify controller images exist
docker images | grep controller
```

### Permission denied on Docker socket

```bash
# Verify user is in docker group
groups  # Should show 'docker'

# If not, add and re-login
sudo usermod -aG docker $USER
exit
# reconnect...
```

## Advantages Over Non-Containerized Deployment

✅ **No Rust installation needed on server**  
✅ **No cargo build on server** (saves time and disk space)  
✅ **Build images anywhere** (including on IPv4 networks)  
✅ **Easy to replicate** (same images work everywhere)  
✅ **Portable** (move between servers easily)  
✅ **Version pinning** (exact reproducible builds)  
✅ **Self-contained** (all dependencies in images)  
✅ **Easy rollbacks** (keep old image tags)  

## Security Considerations

1. **Docker socket mounting**: The kiisrv container has access to Docker, which effectively grants root access. This is necessary for spawning controller containers.

2. **Non-root user**: The kiisrv container runs as a non-root user (uid 1000) for defense in depth.

3. **GitHub API key**: Store in the `apikey` file and mount as a Docker secret.

4. **Firewall**: Only expose ports 80/443. kiisrv runs on 127.0.0.1:3001 internally.

5. **Rate limiting**: Consider adding Nginx rate limiting for public deployments.

## Comparison with Direct Deployment

| Aspect | Containerized | Direct |
|--------|--------------|--------|
| **Setup time** | 15 min (image transfer) | 30-45 min (build on server) |
| **Disk usage** | Same (~10-15GB) | Same |
| **RAM usage** | Same (~300MB idle) | Same |
| **IPv6-only VPS** | ✅ Works (build locally) | ❌ Fails (can't clone from GitHub) |
| **Reproducibility** | ✅ Exact same images | ⚠️ Depends on server state |
| **Updates** | Transfer new image | Rebuild on server |
| **Self-hosting** | ✅ Very easy | ⚠️ Requires Rust knowledge |

## Next Steps

- Set up automatic image builds with GitHub Actions
- Publish images to Docker Hub for easier distribution
- Add health checks to compose file
- Consider multi-architecture builds (ARM64 for Raspberry Pi, etc.)

