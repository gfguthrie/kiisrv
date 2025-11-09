# Production Deployment Checklist

Use this checklist when deploying kiisrv to production.

## Pre-Deployment

### Option A: Using Pre-Built Images (Recommended)

- [ ] Verify images are published
  ```bash
  # Check that workflow has run and images exist
  docker pull ghcr.io/kiibohd/kiisrv-kiisrv:latest
  docker pull ghcr.io/kiibohd/kiisrv-controller-050:latest
  docker pull ghcr.io/kiibohd/kiisrv-controller-057:latest
  ```

- [ ] Optional: Test locally first
  ```bash
  curl -O https://raw.githubusercontent.com/kiibohd/kiisrv/main/compose.ghcr.yaml
  mkdir -p tmp_builds tmp_config && touch config.db stats.db
  docker compose -f compose.ghcr.yaml pull
  docker compose -f compose.ghcr.yaml up
  # Test in browser or with curl
  ```

### Option B: Build Locally (For Custom Builds)

- [ ] Git pull latest code
  ```bash
  git pull origin main
  ```

- [ ] Build images
  ```bash
  docker compose -f compose.prod.yaml build kiisrv
  docker compose -f compose.prod.yaml build controller-050  # LTS
  docker compose -f compose.prod.yaml build controller-057  # Latest
  ```

- [ ] Test locally
  ```bash
  docker compose -f compose.prod.yaml up
  # In another terminal:
  curl http://localhost:3001/stats
  curl http://localhost:3001/versions
  ```

- [ ] Save images (for transfer to server)
  ```bash
  docker save kiisrv-server:latest | gzip > kiisrv-server.tar.gz
  docker save kiisrv-controller-050:latest | gzip > controller-050.tar.gz
  docker save kiisrv-controller-057:latest | gzip > controller-057.tar.gz
  ```

## Server Setup

- [ ] SSH into server
  ```bash
  ssh root@your-server-ip
  # or: ssh -p 2222 youruser@your-server
  ```

- [ ] Install Docker
  ```bash
  curl -fsSL https://get.docker.com | sudo sh
  
  # If using non-root user:
  sudo usermod -aG docker $USER
  # Log out and back in for group to take effect
  ```

- [ ] Create deployment directory
  ```bash
  sudo mkdir -p /opt/kiisrv
  sudo chown $USER:$USER /opt/kiisrv
  cd /opt/kiisrv
  ```

### Option A: Using Pre-Built Images

- [ ] Download compose file
  ```bash
  curl -O https://raw.githubusercontent.com/kiibohd/kiisrv/main/compose.ghcr.yaml
  ```

- [ ] Create runtime files and directories
  ```bash
  mkdir -p tmp_builds tmp_config
  
  # CRITICAL: Create database files as FILES (not directories)
  touch config.db stats.db
  
  # Set proper permissions (container runs as uid 1000)
  chmod 666 config.db stats.db
  chmod 777 tmp_builds tmp_config
  
  # Verify they're files:
  file config.db stats.db
  # Should show: "empty" (not "directory")
  ```

- [ ] Optional: Add GitHub API token
  ```bash
  echo "your_github_token" > apikey
  ```

- [ ] Pull pre-built images
  ```bash
  docker compose -f compose.ghcr.yaml pull
  ```

### Option B: Transfer Custom Built Images

- [ ] Transfer images from local machine
  ```bash
  # On local machine:
  scp -P 2222 *.tar.gz yourserver:/tmp/
  scp -P 2222 compose.prod.yaml yourserver:/opt/kiisrv/
  ```

- [ ] Load images on server
  ```bash
  # On server:
  cd /opt/kiisrv
  docker load < /tmp/kiisrv-server.tar.gz
  docker load < /tmp/controller-050.tar.gz
  docker load < /tmp/controller-057.tar.gz
  ```

- [ ] Create runtime files and directories
  ```bash
  mkdir -p tmp_builds tmp_config
  touch config.db stats.db
  echo "your_github_token" > apikey  # Optional
  ```

## Network Setup

- [ ] Configure firewall
  ```bash
  sudo ufw allow 80/tcp
  sudo ufw allow 443/tcp
  sudo ufw enable
  ```

- [ ] Install Nginx
  ```bash
  sudo apt install -y nginx certbot python3-certbot-nginx
  ```

- [ ] Create Nginx config (see docs/CONTAINERIZED_DEPLOYMENT.md)

- [ ] Test Nginx config
  ```bash
  sudo nginx -t
  sudo systemctl reload nginx
  ```

- [ ] Setup SSL
  ```bash
  sudo certbot --nginx -d configurator.yourdomain.com
  ```

## Start Services

- [ ] Start kiisrv stack
  ```bash
  # If using pre-built images:
  docker compose -f compose.ghcr.yaml up -d
  
  # If using custom built images:
  docker compose -f compose.prod.yaml up -d
  ```

- [ ] Verify containers running
  ```bash
  docker compose -f compose.ghcr.yaml ps
  # Should show kiisrv container running
  ```

- [ ] Check logs
  ```bash
  docker compose -f compose.ghcr.yaml logs -f kiisrv
  # Look for: "Listening on http://0.0.0.0:3001"
  ```

## Verification

- [ ] Test local access
  ```bash
  curl http://localhost:3001/stats
  curl http://localhost:3001/versions
  ```

- [ ] Test public access
  ```bash
  curl https://configurator.yourdomain.com/stats
  curl https://configurator.yourdomain.com/versions
  ```

- [ ] Test firmware build (use configurator client)

- [ ] Check build artifacts created
  ```bash
  ls -la /opt/kiisrv/tmp_builds/
  ```

- [ ] Verify SSL certificate
  ```bash
  sudo certbot certificates
  ```

## Monitoring Setup

- [ ] Test auto-renewal
  ```bash
  sudo certbot renew --dry-run
  ```

- [ ] Setup log rotation (optional)
  ```bash
  # Docker handles container logs automatically
  docker compose -f compose.prod.yaml logs --tail=100
  ```

- [ ] Monitor resource usage
  ```bash
  docker stats --no-stream
  df -h
  ```

## Post-Deployment

- [ ] Document deployment date and versions
  ```bash
  docker images | grep kiisrv
  echo "Deployed $(date)" >> /opt/kiisrv/DEPLOYMENT_LOG.txt
  ```

- [ ] Test from client
  - [ ] Submit build request
  - [ ] Download firmware
  - [ ] Verify .dfu.bin files

- [ ] Setup monitoring alerts (optional)

- [ ] Schedule cleanup job (optional)
  ```bash
  # Add to crontab
  0 2 * * * find /opt/kiisrv/tmp_builds -mtime +7 -delete
  ```

## Rollback Plan

If deployment fails:

- [ ] Keep old images tagged
  ```bash
  docker tag kiisrv-server:latest kiisrv-server:backup-$(date +%Y%m%d)
  ```

- [ ] Test rollback procedure
  ```bash
  docker compose -f compose.prod.yaml down
  docker tag kiisrv-server:backup-20250101 kiisrv-server:latest
  docker compose -f compose.prod.yaml up -d
  ```

## Troubleshooting

**Container won't start:**
```bash
docker compose -f compose.prod.yaml logs kiisrv
docker compose -f compose.prod.yaml ps
```

**Permission errors:**
```bash
ls -la /opt/kiisrv/
sudo chown -R $USER:$USER /opt/kiisrv/tmp_*
```

**Port conflicts:**
```bash
sudo lsof -i :3001
# Change port in compose.prod.yaml if needed
```

**Out of disk space:**
```bash
docker system prune -a
df -h
```

## Update Procedure

### Using Pre-Built Images (Easiest)

- [ ] Pull latest images
  ```bash
  cd /opt/kiisrv
  docker compose -f compose.ghcr.yaml pull
  ```

- [ ] Restart with new images
  ```bash
  docker compose -f compose.ghcr.yaml up -d
  ```

- [ ] Verify update
  ```bash
  curl http://localhost:3001/stats
  docker compose -f compose.ghcr.yaml logs --tail=50 kiisrv
  ```

### Using Custom Built Images

- [ ] Build new images locally
- [ ] Save and transfer to server
- [ ] Load on server
- [ ] Restart containers
  ```bash
  docker compose -f compose.prod.yaml up -d
  ```
- [ ] Verify

## Contact

For issues, see:
- [CONTAINERIZED_DEPLOYMENT.md](./CONTAINERIZED_DEPLOYMENT.md)
- [IMPLEMENTATION_NOTES.md](./IMPLEMENTATION_NOTES.md)
- [CONTAINERIZATION_SUMMARY.md](./CONTAINERIZATION_SUMMARY.md)
- GitHub Issues

