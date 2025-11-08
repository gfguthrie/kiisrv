# KiiSrv Quick Start Guide

Choose your deployment method:

## 🚀 Containerized (Recommended)

**Best for:** Production, self-hosting, IPv6-only servers

### With Pre-Built Images (Fastest!)

```bash
# 1. Install Docker
curl -fsSL https://get.docker.com | sudo sh

# 2. Get kiisrv
git clone https://github.com/kiibohd/kiisrv.git
cd kiisrv

# 3. Create database files (required before first run)
touch config.db stats.db
chmod 666 config.db stats.db  # Allow container to write

# 4. Pull and start (no build needed!)
docker compose -f compose.ghcr.yaml pull
docker compose -f compose.ghcr.yaml up -d

# Done! Server running on http://localhost:3001
curl http://localhost:3001/versions
```

**Time:** 2-3 minutes (vs 20-30 min building locally)

### Build Locally (If Pre-Built Images Unavailable)

```bash
# 1-3. Same as above

# 4. Build and start
docker compose -f compose.prod.yaml build
docker compose -f compose.prod.yaml up -d

# Done! Server running on http://localhost:3001
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
# Requires: Docker only (no Rust, no build tools)
git clone https://github.com/kiibohd/kiisrv.git
cd kiisrv
docker compose -f compose.prod.yaml up -d
```

Point your configurator to `http://localhost:3001`.

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
   mkdir -p tmp_builds tmp_config && touch config.db stats.db
   chmod 666 config.db stats.db  # Allow container to write
   docker compose -f compose.ghcr.yaml pull
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

### IPv6-only server can't clone from GitHub
✅ **Solution:** Build images locally, transfer via rsync or scp  
📖 **Guide:** [docs/CONTAINERIZED_DEPLOYMENT.md](docs/CONTAINERIZED_DEPLOYMENT.md#option-1-build-locally-deploy-anywhere-recommended-for-ipv6-only-servers)

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

