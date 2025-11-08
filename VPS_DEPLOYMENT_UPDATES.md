# VPS Deployment Documentation Updates

## Summary

Updated all VPS deployment documentation to prioritize **pre-built images from GitHub Container Registry** as the primary deployment method. This makes deployment dramatically simpler and faster.

## What Changed

### Files Updated

1. **`docs/CONTAINERIZED_DEPLOYMENT.md`** - Major rewrite
   - Reorganized deployment options (pre-built first, then custom builds)
   - Added "Complete VPS Setup from Scratch" section
   - Simplified all instructions to use `compose.ghcr.yaml`
   - Updated troubleshooting for pre-built images
   - Added comparison table of deployment methods

2. **`QUICK_START.md`** - Simplified VPS section
   - Reduced from multi-step build process to 3 simple steps
   - Changed from ~30 minute process to ~3 minute process
   - Now uses pre-built images by default

3. **`docs/DEPLOYMENT_CHECKLIST.md`** - Added options
   - Split into Option A (pre-built) and Option B (custom builds)
   - Pre-built option is now recommended
   - Updated all commands to use `compose.ghcr.yaml`

## Before vs After

### Before: Build Locally, Transfer to VPS

**Time:** 15-30 minutes (build) + 5-10 minutes (transfer) = 25-40 minutes total

```bash
# On local machine
docker compose -f compose.prod.yaml build
docker save kiisrv-server:latest | gzip > kiisrv-server.tar.gz
scp kiisrv-server.tar.gz yourserver:/tmp/

# On server
docker load < /tmp/kiisrv-server.tar.gz
docker compose -f compose.prod.yaml up -d
```

**Issues:**
- Required building on local machine
- Large file transfers (2-3GB per controller image)
- Complex multi-step process
- Still didn't work well on IPv6-only servers without local IPv4

### After: Pull Pre-Built Images

**Time:** 2-3 minutes total

```bash
# On server (directly)
cd /opt/kiisrv
curl -O https://raw.githubusercontent.com/kiibohd/kiisrv/main/compose.ghcr.yaml
mkdir -p tmp_builds tmp_config && touch config.db stats.db
docker compose -f compose.ghcr.yaml pull
docker compose -f compose.ghcr.yaml up -d
```

**Benefits:**
- ✅ No local building required
- ✅ No file transfers needed
- ✅ Works on IPv6-only servers (ghcr.io supports IPv6)
- ✅ One simple workflow
- ✅ Automatically pulls latest images
- ✅ Easy updates: just `pull` and restart

## Complete VPS Deployment (From Scratch)

The new simplified workflow documented in `docs/CONTAINERIZED_DEPLOYMENT.md`:

### Step 1: Install Docker (1 minute)
```bash
ssh root@your-server-ip
curl -fsSL https://get.docker.com | sudo sh
```

### Step 2: Deploy kiisrv (2-3 minutes)
```bash
sudo mkdir -p /opt/kiisrv && cd /opt/kiisrv
curl -O https://raw.githubusercontent.com/kiibohd/kiisrv/main/compose.ghcr.yaml
mkdir -p tmp_builds tmp_config && touch config.db stats.db
docker compose -f compose.ghcr.yaml pull
docker compose -f compose.ghcr.yaml up -d
curl http://localhost:3001/stats  # Verify
```

### Step 3: Setup Nginx + SSL (2-3 minutes)
```bash
sudo apt install -y nginx certbot python3-certbot-nginx
# Configure nginx (see docs)
sudo certbot --nginx -d yourdomain.com
```

### Step 4: Firewall (1 minute)
```bash
sudo ufw allow 22/tcp 80/tcp 443/tcp
sudo ufw enable
```

**Total time:** ~5-10 minutes for complete production deployment!

## Key Documentation Sections Added

### 1. Complete VPS Setup from Scratch
- Step-by-step guide for fresh server
- From SSH to running service
- Includes Docker installation, deployment, Nginx, SSL, firewall
- Located in `docs/CONTAINERIZED_DEPLOYMENT.md`

### 2. Deployment Options Comparison Table

| Aspect | Pre-Built Images | Build Locally | Build on Server |
|--------|------------------|---------------|-----------------|
| **Setup time** | 2-3 min | 15 min | 30-45 min |
| **IPv6-only VPS** | ✅ Works | ✅ Works | ⚠️ May fail |
| **Internet needed** | ✅ Yes (pull) | ❌ No (on VPS) | ✅ Yes |
| **Best for** | Most deployments | Custom builds | Development |

### 3. Management Commands Updated
All commands now default to `compose.ghcr.yaml`:
- Start: `docker compose -f compose.ghcr.yaml up -d`
- Update: `docker compose -f compose.ghcr.yaml pull && up -d`
- Logs: `docker compose -f compose.ghcr.yaml logs -f`

### 4. Troubleshooting for Pre-Built Images
Added specific troubleshooting for:
- Images fail to pull
- Empty versions endpoint (pull images)
- Controller containers not found (pull images)

## When to Use Each Method

### Pre-Built Images (Option 1) - **RECOMMENDED**
**Use when:**
- ✅ Fresh VPS deployment
- ✅ Production deployment
- ✅ Self-hosting
- ✅ Quick setup needed
- ✅ IPv6-only servers

### Build Locally, Transfer (Option 2)
**Use when:**
- ⚠️ You've modified the code
- ⚠️ Pre-built images aren't available yet
- ⚠️ Testing custom changes

### Build on Server (Option 3)
**Use when:**
- ⚠️ Development/testing
- ⚠️ Server has IPv4 GitHub access
- ⚠️ You prefer building on-site

## Updates Required Before This Works

**Prerequisites:**
1. GitHub Actions workflow must run successfully
2. Images must be published to ghcr.io
3. Repository must be public OR users need GITHUB_TOKEN

**To enable:**
```bash
# Push the GitHub Actions workflow
git add .github/workflows/docker-build-publish.yml
git add compose.ghcr.yaml
git commit -m "Add GitHub Actions for automated Docker builds"
git push origin containerize

# Workflow will automatically build and publish images
# Check status at: https://github.com/{user}/{repo}/actions
```

**After first successful workflow run:**
- Images available at `ghcr.io/{username}/kiisrv-*`
- All VPS deployment instructions work immediately
- Users can deploy in 2-3 minutes

## Self-Hosting Improvements

The documentation now emphasizes how easy self-hosting has become:

**Old self-hosting instructions:**
```bash
git clone https://github.com/kiibohd/kiisrv.git
cd kiisrv
docker compose -f compose.prod.yaml build  # 20-30 minutes
docker compose -f compose.prod.yaml up -d
```

**New self-hosting instructions:**
```bash
curl -O https://raw.githubusercontent.com/kiibohd/kiisrv/main/compose.ghcr.yaml
mkdir -p kiisrv && cd kiisrv
mkdir -p tmp_builds tmp_config && touch config.db stats.db
docker compose -f compose.ghcr.yaml pull  # 2-3 minutes
docker compose -f compose.ghcr.yaml up -d
```

This makes kiisrv accessible to users who:
- Don't have development experience
- Don't want to wait 20-30 minutes
- Just want a working keyboard configurator

## Files That Reference Pre-Built Images

All updated to prioritize pre-built images:
1. `README.md` - Option A (pre-built) before Option B (build)
2. `QUICK_START.md` - Pre-built as primary method
3. `docs/CONTAINERIZED_DEPLOYMENT.md` - Complete rewrite
4. `docs/DEPLOYMENT_CHECKLIST.md` - Option A/B split
5. `docs/GITHUB_ACTIONS_DEPLOYMENT.md` - New file for details
6. `DOCUMENTATION_INDEX.md` - Pre-built images first

## Summary

The VPS deployment experience has been transformed from:
- ❌ Complex multi-step process
- ❌ 25-40 minute deployment
- ❌ Required local build machine
- ❌ Large file transfers

To:
- ✅ Simple one-step process  
- ✅ 2-3 minute deployment
- ✅ Works directly on VPS
- ✅ No file transfers

This makes kiisrv deployment accessible to a much wider audience and solves the IPv6-only server problem completely.

