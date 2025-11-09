# KiiSrv Quick Start Guide

Choose your deployment method:

## 🚀 Containerized (Recommended)

**Best for:** Production, self-hosting, IPv6-only servers

### With Pre-Built Images (Fastest!)

**Minimal setup - just download compose file and helper script!**

```bash
# 1. Install Docker
curl -fsSL https://get.docker.com | sudo sh

# 2. Create directory and download files
mkdir -p ~/kiisrv && cd ~/kiisrv
# For IPv6-only servers, use compose.dockerhub.yaml instead (better compatibility)
curl -O https://raw.githubusercontent.com/kiibohd/kiisrv/main/compose.ghcr.yaml
curl -O https://raw.githubusercontent.com/kiibohd/kiisrv/main/pull-and-tag.sh
chmod +x pull-and-tag.sh

# 3. Create required files
mkdir -p tmp_builds tmp_config
touch config.db stats.db
chmod 666 config.db stats.db  # Allow container to write

# 4. Pull images and tag them correctly
# By default, pulls only controller-057 (latest). Use "all" to pull all versions.
./pull-and-tag.sh compose.ghcr.yaml kiibohd latest

# Optional: Pull all controller versions (takes longer, more disk space)
# ./pull-and-tag.sh compose.ghcr.yaml kiibohd latest all

# 5. Start kiisrv
docker compose -f compose.ghcr.yaml up -d

# Done! Server running on http://localhost:3001
curl http://localhost:3001/versions
```

**Time:** 1-2 minutes for latest controller, 2-3 minutes for all (vs 20-30 min building locally)  
**Space:** ~3GB for latest controller, ~15GB for all controllers

**Why the `pull-and-tag.sh` script?** kiisrv expects images named `kiisrv-controller-057`, but registries store them as `ghcr.io/owner/kiisrv-controller-057:tag`. The script automatically creates the correct local tags.

**Controller versions:**
- **Default (057 only)**: Fastest, recommended for most users - supports latest keyboards
- **All controllers**: Use if you need LTS (050) or specific older versions
- **Custom**: Specify versions like `./pull-and-tag.sh compose.ghcr.yaml kiibohd latest "050 057"`

### Build Locally (If Pre-Built Images Unavailable)

**Requires cloning the repo for source code and Dockerfiles.**

```bash
# 1. Install Docker
curl -fsSL https://get.docker.com | sudo sh

# 2. Clone the repository
git clone https://github.com/kiibohd/kiisrv.git
cd kiisrv

# 3. Create required files
mkdir -p tmp_builds tmp_config
touch config.db stats.db
chmod 666 config.db stats.db

# 4. Build and start (20-30 minutes first time)
docker compose -f compose.prod.yaml build
docker compose -f compose.prod.yaml up -d

# Done! Server running on http://localhost:3001
curl http://localhost:3001/versions
```

**See:** 
- [docs/GITHUB_ACTIONS_DEPLOYMENT.md](docs/GITHUB_ACTIONS_DEPLOYMENT.md) - Pre-built images
- [docs/CONTAINERIZED_DEPLOYMENT.md](docs/CONTAINERIZED_DEPLOYMENT.md) - Full deployment guide

**Important:** Create empty `config.db` and `stats.db` files before first run to prevent Docker from creating them as directories.

---

## 🛠️ Local Development

**Best for:** Contributing to kiisrv, debugging

```bash
# 1. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Build controller images
docker compose build controller-057

# 3. Run server
cargo run

# Server running on http://0.0.0.0:3001
```

**See:** [README.md](README.md#local-development)

---

## 📦 Self-Hosting for End Users

If you just want to run your own kiisrv instance:

```bash
# Requires: Docker only (no Rust, no build tools, no git clone!)
mkdir -p ~/kiisrv && cd ~/kiisrv
curl -O https://raw.githubusercontent.com/kiibohd/kiisrv/main/compose.ghcr.yaml
curl -O https://raw.githubusercontent.com/kiibohd/kiisrv/main/pull-and-tag.sh
chmod +x pull-and-tag.sh
mkdir -p tmp_builds tmp_config && touch config.db stats.db
chmod 666 config.db stats.db
./pull-and-tag.sh compose.ghcr.yaml kiibohd latest  # Pulls controller-057 only
docker compose -f compose.ghcr.yaml up -d
```

Point your configurator to `http://localhost:3001`.

**That's it!** No repo cloning, no building - just pull and run.

---

## 🌐 Production VPS Deployment

**For Hetzner or any VPS (fresh server):**

1. **SSH to your server and install Docker**:
   ```bash
   ssh root@your-server-ip
   curl -fsSL https://get.docker.com | sudo sh
   ```

2. **Deploy kiisrv** (2-3 minutes):
   ```bash
   sudo mkdir -p /opt/kiisrv && cd /opt/kiisrv
   curl -O https://raw.githubusercontent.com/kiibohd/kiisrv/main/compose.ghcr.yaml
   curl -O https://raw.githubusercontent.com/kiibohd/kiisrv/main/pull-and-tag.sh
   chmod +x pull-and-tag.sh
   mkdir -p tmp_builds tmp_config && touch config.db stats.db
   chmod 666 config.db stats.db  # Allow container to write
   # Pull only latest controller (057) - fastest option
   ./pull-and-tag.sh compose.ghcr.yaml kiibohd latest
   # Or pull all controllers: ./pull-and-tag.sh compose.ghcr.yaml kiibohd latest all
   docker compose -f compose.ghcr.yaml up -d
   ```

3. **Setup Nginx + SSL**:
   ```bash
   sudo apt install -y nginx certbot python3-certbot-nginx
   # Configure Nginx (see full guide)
   sudo certbot --nginx -d yourdomain.com
   ```

**That's it!** Pre-built images make deployment super fast.

**See:** [docs/CONTAINERIZED_DEPLOYMENT.md](docs/CONTAINERIZED_DEPLOYMENT.md) for complete setup guide

---

## Common Issues

### "Unable to find image 'kiisrv-controller-057:latest'"
✅ **Solution:** Use the `pull-and-tag.sh` script after pulling images  
**Why:** kiisrv expects simple image names, but registries use full paths  
```bash
./pull-and-tag.sh compose.ghcr.yaml kiibohd latest
```

### IPv6-only server can't pull images
✅ **Solution:** Use pre-built images (ghcr.io supports IPv6) or build locally and transfer  
📖 **Guide:** [docs/CONTAINERIZED_DEPLOYMENT.md](docs/CONTAINERIZED_DEPLOYMENT.md#deployment-options)

### "cargo: command not found"
✅ **Solution:** Install Rust or use containerized deployment  
📖 **Guide:** [README.md](README.md#local-development)

### "docker: command not found"
✅ **Solution:** Install Docker
```bash
curl -fsSL https://get.docker.com | sudo sh
```

---

## Next Steps

- 📚 **Documentation Index:** [DOCUMENTATION_INDEX.md](DOCUMENTATION_INDEX.md)
- 🐳 **Containerized Guide:** [docs/CONTAINERIZED_DEPLOYMENT.md](docs/CONTAINERIZED_DEPLOYMENT.md)
- 🛠️ **Development Guide:** [README.md](README.md)
- 🏃 **Running Locally:** [docs/RUNNING_LOCALLY.md](docs/RUNNING_LOCALLY.md)
- 📖 **Modernization Details:** [docs/MODERNIZATION.md](docs/MODERNIZATION.md)

