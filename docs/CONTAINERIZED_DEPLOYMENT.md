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

### Option 1: Using Pre-Built Images (Recommended - Fastest!)

**When to use:**
- Fresh VPS deployment
- Quick setup (2-3 minutes vs 20-30 minutes)
- Any server with Docker and internet access (IPv4 or IPv6)

**Steps:**

```bash
# 1. SSH into your VPS
ssh -p 2222 youruser@your-server

# 2. Create deployment directory
sudo mkdir -p /opt/kiisrv
sudo chown $USER:$USER /opt/kiisrv
cd /opt/kiisrv

# 3. Download compose file for pre-built images
curl -O https://raw.githubusercontent.com/kiibohd/kiisrv/main/compose.ghcr.yaml

# 4. Create required files/directories
mkdir -p tmp_builds tmp_config

# IMPORTANT: Create database files BEFORE starting containers
# If these don't exist, Docker will create them as directories instead
touch config.db stats.db

# Optional: Add GitHub API token (avoid rate limits)
echo "your_github_token_here" > apikey

# 5. Pull pre-built images and start
docker compose -f compose.ghcr.yaml pull
docker compose -f compose.ghcr.yaml up -d

# 6. Verify
docker compose -f compose.ghcr.yaml ps
curl http://localhost:3001/stats
curl http://localhost:3001/versions
```

**That's it!** No local building, no image transfers, no waiting 30 minutes.

### Option 2: Build Locally, Deploy Anywhere (For Custom Builds)

**When to use:**
- You've modified the code
- Pre-built images aren't available yet
- You want to test local changes before deploying

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

# 4. Copy compose file
scp -P 2222 compose.prod.yaml youruser@your-server:/opt/kiisrv/

# 5. On the server: Load images
ssh -p 2222 youruser@your-server
cd /opt/kiisrv
docker load < /tmp/kiisrv-server.tar.gz
docker load < /tmp/controller-050.tar.gz
docker load < /tmp/controller-057.tar.gz

# 6. Create required files/directories
mkdir -p tmp_builds tmp_config
touch config.db stats.db
echo "your_github_token_here" > apikey  # Optional

# 7. Start the stack
docker compose -f compose.prod.yaml up -d

# 8. Verify
docker compose -f compose.prod.yaml ps
curl http://localhost:3001/stats
```

### Option 3: Build on Server (If Server Has IPv4 GitHub Access)

**When to use:**
- Your server has IPv4 connectivity and can access GitHub
- You prefer building directly on the deployment server

```bash
# On the server
git clone https://github.com/kiibohd/kiisrv.git /opt/kiisrv
cd /opt/kiisrv

# Create required files
mkdir -p tmp_builds tmp_config
touch config.db stats.db

# Build images (takes 20-30 minutes)
docker compose -f compose.prod.yaml build

# Start the stack
docker compose -f compose.prod.yaml up -d
```

## Complete VPS Setup from Scratch

This guide walks through setting up kiisrv on a fresh VPS (Hetzner, DigitalOcean, AWS, etc.).

### Step 1: Install Docker

```bash
# SSH into your fresh VPS
ssh root@your-server-ip

# Install Docker (works on Debian 12, Ubuntu 22.04+, etc.)
curl -fsSL https://get.docker.com | sudo sh

# Add your user to docker group (if using non-root user)
sudo usermod -aG docker $USER

# Verify installation
docker --version
docker compose version

# If using non-root user, log out and back in for group to take effect
```

### Step 2: Deploy kiisrv

```bash
# Create deployment directory
sudo mkdir -p /opt/kiisrv
sudo chown $USER:$USER /opt/kiisrv
cd /opt/kiisrv

# Download compose file for pre-built images
curl -O https://raw.githubusercontent.com/kiibohd/kiisrv/main/compose.ghcr.yaml

# Create runtime directories
mkdir -p tmp_builds tmp_config

# CRITICAL: Create database files as FILES (not directories)
touch config.db stats.db

# Set proper permissions (container runs as uid 1000)
chmod 666 config.db stats.db
chmod 777 tmp_builds tmp_config

# Optional: Add GitHub API key to avoid rate limits
echo "your_github_token" > apikey

# Pull pre-built images (2-3 minutes)
docker compose -f compose.ghcr.yaml pull

# Start kiisrv
docker compose -f compose.ghcr.yaml up -d

# Verify it's running
docker compose -f compose.ghcr.yaml ps
curl http://localhost:3001/stats
```

You should see JSON output from the stats endpoint. kiisrv is now running on port 3001!

### Step 3: Setup Nginx Reverse Proxy

```bash
# Install Nginx and Certbot
sudo apt update
sudo apt install -y nginx certbot python3-certbot-nginx

# Create Nginx config
sudo tee /etc/nginx/sites-available/kiisrv << 'EOF'
server {
    listen 80;
    listen [::]:80;
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

# Setup SSL (free certificate from Let's Encrypt)
sudo certbot --nginx -d configurator.yourdomain.com

# Certbot will automatically:
# - Obtain SSL certificate
# - Update Nginx config for HTTPS
# - Setup automatic renewal
```

### Step 4: Firewall Configuration

```bash
# If using UFW (Ubuntu/Debian)
sudo ufw allow 22/tcp    # SSH (if not already allowed)
sudo ufw allow 80/tcp    # HTTP
sudo ufw allow 443/tcp   # HTTPS
sudo ufw enable
sudo ufw status

# Verify firewall rules
sudo ufw status verbose
```

## Management Commands

All commands assume you're in `/opt/kiisrv` and using `compose.ghcr.yaml` (pre-built images).

### Start/Stop/Restart

```bash
cd /opt/kiisrv

# Start
docker compose -f compose.ghcr.yaml up -d

# Stop
docker compose -f compose.ghcr.yaml down

# Restart
docker compose -f compose.ghcr.yaml restart

# Restart just kiisrv (not controllers)
docker compose -f compose.ghcr.yaml restart kiisrv
```

### View Logs

```bash
# All services
docker compose -f compose.ghcr.yaml logs -f

# Just kiisrv
docker compose -f compose.ghcr.yaml logs -f kiisrv

# Last 100 lines
docker compose -f compose.ghcr.yaml logs --tail=100 kiisrv
```

### Updates (Pull New Images)

```bash
cd /opt/kiisrv

# Pull latest images
docker compose -f compose.ghcr.yaml pull

# Restart with new images
docker compose -f compose.ghcr.yaml up -d

# Verify update
curl http://localhost:3001/stats
```

**Automatic updates with Watchtower** (optional):
```bash
# Install Watchtower to auto-update daily
docker run -d \
  --name watchtower \
  --restart unless-stopped \
  -v /var/run/docker.sock:/var/run/docker.sock \
  containrrr/watchtower \
  --interval 86400 \
  --cleanup \
  kiisrv
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

If you want to self-host kiisrv on your local machine:

```bash
# 1. Install Docker
curl -fsSL https://get.docker.com | sudo sh

# 2. Create directory and download compose file
mkdir -p ~/kiisrv && cd ~/kiisrv
curl -O https://raw.githubusercontent.com/kiibohd/kiisrv/main/compose.ghcr.yaml

# 3. Create required files
mkdir -p tmp_builds tmp_config
touch config.db stats.db

# 4. Pull images and start (2-3 minutes)
docker compose -f compose.ghcr.yaml pull
docker compose -f compose.ghcr.yaml up -d

# 5. Access at http://localhost:3001
curl http://localhost:3001/stats
```

That's it! Point your configurator to `http://localhost:3001`.

**To stop:** `docker compose -f compose.ghcr.yaml down`  
**To update:** `docker compose -f compose.ghcr.yaml pull && docker compose -f compose.ghcr.yaml up -d`

## Troubleshooting

### kiisrv container won't start

```bash
# Check logs
docker compose -f compose.ghcr.yaml logs kiisrv

# Common issues:
# - Port 3001 already in use: Change KIISRV_PORT in compose.ghcr.yaml
# - Database permission errors: Check ownership of *.db files
# - Docker socket permission denied: User needs to be in docker group
```

### Database permission errors (ReadOnly error)

```bash
# Error: "attempt to write a readonly database"
# This happens when database files have wrong permissions

# Fix:
chmod 666 config.db stats.db
chmod 777 tmp_builds tmp_config

# Restart
docker compose -f compose.ghcr.yaml restart kiisrv
```

### Empty versions endpoint returns {}

```bash
# Check if databases exist
docker exec kiisrv ls -la /app/*.db

# Check if images are visible
docker exec kiisrv docker images | grep kiisrv-controller

# If no images found, pull them:
docker compose -f compose.ghcr.yaml pull

# Restart kiisrv
docker compose -f compose.ghcr.yaml restart kiisrv
```

### Database files become directories

```bash
# This happens if files don't exist before 'docker compose up'
# Fix:
cd /opt/kiisrv
docker compose -f compose.ghcr.yaml down
rm -rf config.db stats.db  # Remove directories
touch config.db stats.db   # Create empty files
docker compose -f compose.ghcr.yaml up -d
```

### Images fail to pull

```bash
# Check if you can reach GitHub Container Registry
docker pull ghcr.io/kiibohd/kiisrv-kiisrv:latest

# If this fails, check:
# 1. Internet connectivity: ping github.com
# 2. DNS resolution: nslookup ghcr.io
# 3. Firewall rules: sudo ufw status

# If images don't exist yet (before CI/CD runs), use Option 2 or 3 above
```

### Controller containers not found

```bash
# List available images
docker images | grep kiisrv

# Pull missing controller images
docker compose -f compose.ghcr.yaml pull

# Verify they're now available
docker images | grep controller
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

## Comparison of Deployment Methods

| Aspect | Pre-Built Images | Build Locally | Build on Server |
|--------|------------------|---------------|-----------------|
| **Setup time** | 2-3 min | 15 min (build + transfer) | 30-45 min |
| **Disk usage** | ~10-15GB | Same | Same |
| **RAM usage** | ~300MB idle | Same | Same |
| **IPv4 needed** | ❌ No | ❌ No | ✅ Yes |
| **IPv6-only VPS** | ✅ Works | ✅ Works | ⚠️ May fail |
| **Internet needed** | ✅ Yes (pull images) | ❌ No (on VPS) | ✅ Yes |
| **Reproducibility** | ✅ Exact same | ✅ Exact same | ⚠️ Varies |
| **Updates** | Pull new images | Build and transfer | Rebuild on server |
| **Self-hosting** | ✅ Easiest | ⚠️ Moderate | ⚠️ Complex |
| **Best for** | Most deployments | Custom builds | Development |

## Next Steps

- Set up automatic image builds with GitHub Actions
- Publish images to Docker Hub for easier distribution
- Add health checks to compose file
- Consider multi-architecture builds (ARM64 for Raspberry Pi, etc.)

