# KiiSrv Quick Start Guide

Choose your deployment method:

## 🚀 Containerized (Recommended)

**Best for:** Production, self-hosting, IPv6-only servers

```bash
# 1. Install Docker
curl -fsSL https://get.docker.com | sudo sh

# 2. Get kiisrv
git clone https://github.com/kiibohd/kiisrv.git
cd kiisrv

# 3. Create database files (required before first run)
touch config.db stats.db

# 4. Build and start
docker compose -f compose.prod.yaml build
docker compose -f compose.prod.yaml up -d

# Done! Server running on http://localhost:3001
curl http://localhost:3001/versions
```

**See:** [docs/CONTAINERIZED_DEPLOYMENT.md](docs/CONTAINERIZED_DEPLOYMENT.md)

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

**For Hetzner or any VPS:**

1. **Build images locally** (on your Mac/workstation):
   ```bash
   docker compose -f compose.prod.yaml build
   docker save kiisrv-server:latest | gzip > kiisrv-server.tar.gz
   docker save kiisrv-controller-057:latest | gzip > controller-057.tar.gz
   ```

2. **Transfer to server**:
   ```bash
   scp kiisrv-server.tar.gz yourserver:/tmp/
   scp controller-057.tar.gz yourserver:/tmp/
   ```

3. **Load and run on server**:
   ```bash
   docker load < /tmp/kiisrv-server.tar.gz
   docker load < /tmp/controller-057.tar.gz
   docker compose -f compose.prod.yaml up -d
   ```

4. **Setup Nginx reverse proxy** (see containerized deployment guide)

**See:** [docs/CONTAINERIZED_DEPLOYMENT.md](docs/CONTAINERIZED_DEPLOYMENT.md)

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

