# Production Deployment Checklist

Use this checklist when deploying kiisrv to production.

## Pre-Deployment (Local Machine)

- [ ] Git pull latest code
  ```bash
  git pull origin modernize
  ```

- [ ] Optional: Add GitHub API token (to avoid rate limits)
  ```bash
  echo "ghp_yourtoken" > apikey
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

- [ ] Save images (if deploying to IPv6-only server)
  ```bash
  docker save kiisrv-server:latest | gzip > kiisrv-server.tar.gz
  docker save kiisrv-controller-050:latest | gzip > controller-050.tar.gz
  docker save kiisrv-controller-057:latest | gzip > controller-057.tar.gz
  ```

## Server Setup

- [ ] Install Docker
  ```bash
  curl -fsSL https://get.docker.com | sudo sh
  sudo usermod -aG docker $USER
  # Log out and back in
  ```

- [ ] Create deployment directory
  ```bash
  sudo mkdir -p /opt/kiisrv
  sudo chown $USER:$USER /opt/kiisrv
  ```

- [ ] Transfer files to server
  ```bash
  # Images
  scp -P 2222 *.tar.gz yourserver:/tmp/
  
  # Config and data
  rsync -avz -e "ssh -p 2222" \
    compose.prod.yaml \
    layouts/ \
    schema/ \
    apikey \
    yourserver:/opt/kiisrv/
  ```

- [ ] Load images on server
  ```bash
  ssh yourserver
  cd /opt/kiisrv
  docker load < /tmp/kiisrv-server.tar.gz
  docker load < /tmp/controller-050.tar.gz
  docker load < /tmp/controller-057.tar.gz
  ```

- [ ] Create runtime directories
  ```bash
  mkdir -p tmp_builds tmp_config
  ```

- [ ] Create database files (BEFORE docker compose up!)
  ```bash
  # CRITICAL: Must be files, not directories
  touch config.db stats.db
  # Verify they're files:
  file config.db stats.db
  # Should show: "empty" or "SQLite database"
  # NOT "directory"
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
  docker compose -f compose.prod.yaml up -d
  ```

- [ ] Verify containers running
  ```bash
  docker compose -f compose.prod.yaml ps
  ```

- [ ] Check logs
  ```bash
  docker compose -f compose.prod.yaml logs -f kiisrv
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

- [ ] Build new images locally
- [ ] Save and transfer
- [ ] Load on server
- [ ] Stop old containers
- [ ] Start new containers
- [ ] Verify

## Contact

For issues, see:
- [CONTAINERIZED_DEPLOYMENT.md](./CONTAINERIZED_DEPLOYMENT.md)
- [IMPLEMENTATION_NOTES.md](./IMPLEMENTATION_NOTES.md)
- [CONTAINERIZATION_SUMMARY.md](./CONTAINERIZATION_SUMMARY.md)
- GitHub Issues

