# GitHub Actions Automated Builds

## Overview

kiisrv uses GitHub Actions to automatically build and publish Docker images to GitHub Container Registry (ghcr.io). This means users can deploy without building images locally.

## Published Images

Images are automatically published on every push to main/master and on tagged releases:

```
ghcr.io/{owner}/kiisrv-kiisrv:latest           # Main server
ghcr.io/{owner}/kiisrv-controller-050:latest   # LTS firmware builder
ghcr.io/{owner}/kiisrv-controller-057:latest   # Latest firmware builder
```

Replace `{owner}` with the GitHub username or organization.

## Quick Deployment (No Build Required!)

### Choosing a Registry

**Docker Hub** (recommended for IPv6-only servers):
- ✅ Excellent IPv6 support globally
- ✅ Works reliably on IPv6-only VPS
- ✅ Use `compose.dockerhub.yaml`

**GitHub Container Registry**:
- ✅ Free for public repos
- ⚠️ IPv6 support varies by region
- ⚠️ May not work on some IPv6-only servers
- ✅ Use `compose.ghcr.yaml`

### Self-Hosting with Pre-Built Images (Docker Hub)

Instead of building locally, pull pre-built images from Docker Hub:

```bash
# 1. Create deployment directory
mkdir -p ~/kiisrv && cd ~/kiisrv

# 2. Download compose file for Docker Hub
curl -O https://raw.githubusercontent.com/kiibohd/kiisrv/main/compose.dockerhub.yaml

# 3. Edit to use your Docker Hub username
sed -i 's/your-dockerhub-username/YOUR_USERNAME/g' compose.dockerhub.yaml

# Or create a custom compose file:
cat > compose.dockerhub.yaml << 'EOF'
services:
  kiisrv:
    image: ghcr.io/kiibohd/kiisrv-kiisrv:latest
    container_name: kiisrv
    ports:
      - "3001:3001"
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock
      - ./config.db:/app/config.db
      - ./stats.db:/app/stats.db
      - ./tmp_builds:/app/tmp_builds
      - ./tmp_config:/app/tmp_config
      - ./apikey:/app/apikey:ro
    environment:
      - KIISRV_HOST=0.0.0.0
      - KIISRV_PORT=3001
      - HOST_TMP_CONFIG=${PWD}/tmp_config
      - HOST_TMP_BUILDS=${PWD}/tmp_builds
    networks:
      - kiisrv
    restart: unless-stopped

  controller-050:
    image: ghcr.io/kiibohd/kiisrv-controller-050:latest
    profiles:
      - manual
    networks:
      - kiisrv

  controller-057:
    image: ghcr.io/kiibohd/kiisrv-controller-057:latest
    profiles:
      - manual
    networks:
      - kiisrv

networks:
  kiisrv:
    driver: bridge
EOF

# 4. Create required files
mkdir -p tmp_builds tmp_config
touch config.db stats.db
chmod 666 config.db stats.db  # Allow container to write
echo "your_github_token" > apikey  # Optional

# 5. Pull and start (no build needed!)
docker compose -f compose.ghcr.yaml pull
docker compose -f compose.ghcr.yaml up -d

# 6. Verify
curl http://localhost:3001/stats
```

**That's it!** No Rust, no build tools, no 20-minute build process.

## For Developers: Setting Up GitHub Actions

### 1. Enable GitHub Container Registry

GitHub Container Registry is automatically available for public repositories. For private repos, you may need to enable it in your organization settings.

### 2. Repository Secrets (Optional)

For Docker Hub publishing (commented out by default):

1. Go to your repository → Settings → Secrets and variables → Actions
2. Add secrets:
   - `DOCKERHUB_USERNAME`: Your Docker Hub username
   - `DOCKERHUB_TOKEN`: Docker Hub access token

### 3. Workflow Triggers

The workflow runs on:
- **Push to main/master/containerize**: Builds and publishes `latest` tag
- **Tagged release** (`v*`): Builds and publishes version tags
- **Pull requests**: Builds only (doesn't push)
- **Manual trigger**: Via GitHub Actions UI

### 4. Published Tags

Images are tagged with:
- `latest` - Latest build from default branch
- `<branch-name>` - Builds from specific branches
- `v1.2.3` - Semantic version tags (if you tag releases)
- `sha-<git-sha>` - Specific commit SHA

## Deployment Scenarios

### Scenario 1: End User Self-Hosting

**Before (with local build):**
```bash
git clone https://github.com/kiibohd/kiisrv.git
cd kiisrv
docker compose -f compose.prod.yaml build  # 20-30 minutes
docker compose -f compose.prod.yaml up -d
```

**After (with pre-built images):**
```bash
curl -O https://raw.githubusercontent.com/kiibohd/kiisrv/main/compose.ghcr.yaml
docker compose -f compose.ghcr.yaml pull  # 2-3 minutes
docker compose -f compose.ghcr.yaml up -d
```

### Scenario 2: Production VPS Deployment

```bash
# On your VPS
cd /opt/kiisrv

# Use pre-built images
docker compose -f compose.ghcr.yaml pull
docker compose -f compose.ghcr.yaml up -d

# Setup Nginx (see CONTAINERIZED_DEPLOYMENT.md)
```

### Scenario 3: IPv6-only Server

**Before:** Had to build locally and transfer images

**After:** 
```bash
# On IPv6-only server (can pull from ghcr.io over IPv6)
docker compose -f compose.ghcr.yaml pull
docker compose -f compose.ghcr.yaml up -d
```

GitHub Container Registry supports IPv6, so this should work on IPv6-only servers!

### Scenario 4: Specific Version Deployment

```bash
# Pull a specific version
docker pull ghcr.io/kiibohd/kiisrv-kiisrv:v1.0.0
docker pull ghcr.io/kiibohd/kiisrv-controller-057:v1.0.0

# Or use in compose file
services:
  kiisrv:
    image: ghcr.io/kiibohd/kiisrv-kiisrv:v1.0.0
```

## Benefits

✅ **No build tools needed** - Just Docker  
✅ **Fast deployment** - 2-3 min download vs 20-30 min build  
✅ **Lower resources** - No need for 8GB RAM to build  
✅ **Consistent images** - Everyone uses the same build  
✅ **IPv6 compatible** - ghcr.io supports IPv6  
✅ **Version pinning** - Use specific version tags  
✅ **Easy rollback** - Pull previous version tag  

## Advanced: Multi-Platform Builds

To build for both AMD64 and ARM64 (for Raspberry Pi, Apple Silicon):

```yaml
# In .github/workflows/docker-build-publish.yml
- name: Build and push Docker image
  uses: docker/build-push-action@v5
  with:
    platforms: linux/amd64,linux/arm64  # Add ARM64
```

**Note:** ARM builds take significantly longer in CI. Consider building only on tagged releases.

## Monitoring Builds

1. Go to your repository on GitHub
2. Click **Actions** tab
3. View workflow runs, logs, and build status

## Updating Images

### Manual Update
```bash
docker compose -f compose.ghcr.yaml pull
docker compose -f compose.ghcr.yaml up -d
```

### Automated Updates with Watchtower
```bash
docker run -d \
  --name watchtower \
  -v /var/run/docker.sock:/var/run/docker.sock \
  containrrr/watchtower \
  --interval 86400 \
  kiisrv
```

This checks for new images daily and auto-updates.

## Troubleshooting

### Image Pull Fails

**Error:** `unauthorized: unauthenticated`

**Solution:** The image may be private or doesn't exist yet. Check:
```bash
# List available images (requires GitHub CLI or browse on GitHub)
gh api /user/packages/container/kiisrv-kiisrv/versions

# Or visit: https://github.com/orgs/{owner}/packages
```

### Rate Limiting

GitHub Container Registry has generous rate limits:
- **Authenticated**: 15,000 pulls per hour
- **Anonymous**: 1,000 pulls per hour

For production, consider:
```bash
# Login to avoid rate limits
echo $GITHUB_TOKEN | docker login ghcr.io -u USERNAME --password-stdin
docker compose -f compose.ghcr.yaml pull
```

### Build Failures in CI

Check the Actions tab for error logs. Common issues:
- **Out of disk space**: GitHub runners have ~14GB free
- **Timeout**: Builds taking >6 hours (free tier limit)
- **Dependency fetch failures**: Transient network issues, re-run workflow

## Cost

**GitHub Container Registry:**
- ✅ **Free** for public repositories
- ✅ **500MB free** for private repositories
- ✅ **Unlimited bandwidth** for public packages

**GitHub Actions:**
- ✅ **Free** for public repositories (unlimited minutes)
- Private repos get 2,000 minutes/month free

## Security

**Public images:**
- Anyone can pull without authentication
- Suitable for open-source projects

**Private images:**
- Require authentication to pull
- Set `packages` permissions in workflows

**Scanning:**
Consider adding vulnerability scanning:
```yaml
- name: Run Trivy vulnerability scanner
  uses: aquasecurity/trivy-action@master
  with:
    image-ref: ${{ env.REGISTRY_GHCR }}/kiisrv-kiisrv:latest
    format: 'sarif'
    output: 'trivy-results.sarif'
```

## Next Steps

1. **Update README.md** to mention pre-built images
2. **Create releases** with version tags (v1.0.0, etc.)
3. **Add badges** to README showing build status
4. **Set up automated testing** with published images
5. **Consider Docker Hub** for wider discoverability

## References

- [GitHub Container Registry Docs](https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-container-registry)
- [GitHub Actions Docker Docs](https://docs.github.com/en/actions/publishing-packages/publishing-docker-images)
- [Docker Build Push Action](https://github.com/docker/build-push-action)

