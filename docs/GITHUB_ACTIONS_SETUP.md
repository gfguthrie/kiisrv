# GitHub Actions Setup Summary

## What Was Created

This setup enables automated Docker image builds and publishing to GitHub Container Registry (ghcr.io).

### Files Created

1. **`.github/workflows/docker-build-publish.yml`** - GitHub Actions workflow
   - Builds 6 images: kiisrv + 5 controller versions (050, 054, 055, 056, 057)
   - Publishes to GitHub Container Registry
   - Runs automated tests after build
   - Triggers on push to main/master/containerize and on tags

2. **`compose.ghcr.yaml`** - Simplified compose file for pre-built images
   - Uses images from ghcr.io instead of building locally
   - Drop-in replacement for compose.prod.yaml

3. **`docs/GITHUB_ACTIONS_DEPLOYMENT.md`** - Comprehensive guide
   - How to use pre-built images
   - Deployment scenarios
   - Troubleshooting

### Files Updated

- **`README.md`** - Added pre-built images option, build badge
- **`QUICK_START.md`** - Pre-built images as primary method
- **`DOCUMENTATION_INDEX.md`** - Added GitHub Actions documentation

## How It Works

### Workflow Triggers

```yaml
on:
  push:
    branches: [main, master, containerize]
    tags: ['v*']
  pull_request:
    branches: [main, master]
  workflow_dispatch:
```

**When it runs:**
- Push to main/master/containerize → Build and publish `latest`
- Push a tag like `v1.0.0` → Build and publish version tags
- Pull requests → Build only (no publish)
- Manual trigger → Via GitHub Actions UI

### Images Published

After the workflow runs, images are available at both registries:

**Docker Hub (recommended for IPv6-only servers):**
```
{dockerhub_username}/kiisrv-kiisrv:latest
{dockerhub_username}/kiisrv-controller-050:latest
{dockerhub_username}/kiisrv-controller-054:latest
{dockerhub_username}/kiisrv-controller-055:latest
{dockerhub_username}/kiisrv-controller-056:latest
{dockerhub_username}/kiisrv-controller-057:latest
```

**GitHub Container Registry:**
```
ghcr.io/{github_username}/kiisrv-kiisrv:latest
ghcr.io/{github_username}/kiisrv-controller-050:latest
ghcr.io/{github_username}/kiisrv-controller-054:latest
ghcr.io/{github_username}/kiisrv-controller-055:latest
ghcr.io/{github_username}/kiisrv-controller-056:latest
ghcr.io/{github_username}/kiisrv-controller-057:latest
```

## Next Steps

### 1. Set Up Docker Hub (Required)

1. Go to https://hub.docker.com/ and sign in
2. Go to Account Settings → Security → Personal Access Tokens
3. Click "Generate New Token"
   - Name: `github-actions-kiisrv`
   - Permissions: Read, Write, Delete
4. Copy the token (you can't see it again!)

### 2. Add GitHub Secrets

1. Go to your repo: `https://github.com/{your-username}/kiisrv`
2. Click Settings → Secrets and variables → Actions
3. Add two secrets:
   - `DOCKERHUB_USERNAME` = your Docker Hub username
   - `DOCKERHUB_TOKEN` = the token you copied

### 3. Push to GitHub

```bash
git add .
git commit -m "Add GitHub Actions for automated Docker builds"
git push origin containerize
```

### 4. Wait for Workflow to Complete

1. Go to your repository on GitHub
2. Click the **Actions** tab
3. You'll see "Build and Publish Docker Images" running
4. Initial build takes ~30-45 minutes (builds 6 images in parallel)
5. Subsequent builds are faster (~10-15 min) due to caching

### 5. Verify Images Published

After workflow completes:

```bash
# Pull an image to verify
docker pull ghcr.io/{your_username}/kiisrv-kiisrv:containerize

# Or view on GitHub
# Navigate to: https://github.com/{your_username}/kiisrv/pkgs/container/kiisrv-kiisrv
```

### 6. Test Deployment

The `compose.ghcr.yaml` file has a placeholder for the GitHub username:

```yaml
x-github-username: &github-username kiibohd
```

If your fork is under a different username, update this line:

```yaml
x-github-username: &github-username your-actual-username
```

Or, users can override it when pulling:

```bash
# Edit compose.ghcr.yaml to use your username
sed -i 's/kiibohd/your-username/g' compose.ghcr.yaml
```

**Using Docker Hub (recommended for IPv6-only servers):**
```bash
# On a clean system (or new directory)
curl -O https://raw.githubusercontent.com/{your_username}/kiisrv/containerize/compose.dockerhub.yaml

# Replace username in file
sed -i 's/your-dockerhub-username/{your_dockerhub_username}/g' compose.dockerhub.yaml

# Create required files
mkdir -p tmp_builds tmp_config
touch config.db stats.db
chmod 666 config.db stats.db

# Pull and start
docker compose -f compose.dockerhub.yaml pull
docker compose -f compose.dockerhub.yaml up -d

# Verify
curl http://localhost:3001/stats
```

**Using GitHub Container Registry:**
```bash
curl -O https://raw.githubusercontent.com/{your_username}/kiisrv/containerize/compose.ghcr.yaml

# Replace username in file
sed -i 's/kiibohd/{your_github_username}/g' compose.ghcr.yaml

# Create and start (same as above)
docker compose -f compose.ghcr.yaml pull
docker compose -f compose.ghcr.yaml up -d
```

## For End Users

Once images are published, end users can deploy with:

```bash
# Quick deployment (no build needed!)
curl -O https://raw.githubusercontent.com/kiibohd/kiisrv/main/compose.ghcr.yaml
mkdir -p tmp_builds tmp_config && touch config.db stats.db
docker compose -f compose.ghcr.yaml pull
docker compose -f compose.ghcr.yaml up -d
```

**Benefits:**
- ✅ 2-3 minutes vs 20-30 minutes
- ✅ No Rust required
- ✅ No 8GB RAM for builds
- ✅ Works on IPv6-only servers (ghcr.io supports IPv6)

## Creating Releases

To publish version-tagged images:

```bash
# Tag a release
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0

# Workflow automatically builds and publishes:
# - ghcr.io/{user}/kiisrv-kiisrv:v1.0.0
# - ghcr.io/{user}/kiisrv-kiisrv:v1.0
# - ghcr.io/{user}/kiisrv-kiisrv:v1
# - ghcr.io/{user}/kiisrv-kiisrv:latest
```

## Customization

### Add More Controller Versions

Edit `.github/workflows/docker-build-publish.yml`:

```yaml
strategy:
  matrix:
    include:
      # ... existing entries ...
      - image: controller-058
        dockerfile: Dockerfile
        target: controller
        context: .
        build-args: |
          TAG=v0.5.8
```

Also update `compose.ghcr.yaml` to include the new controller.

### Publish to Docker Hub (Optional)

1. Add Docker Hub credentials to GitHub Secrets:
   - `DOCKERHUB_USERNAME`
   - `DOCKERHUB_TOKEN`

2. Uncomment the Docker Hub login section in the workflow:

```yaml
- name: Log in to Docker Hub
  if: github.event_name != 'pull_request'
  uses: docker/login-action@v3
  with:
    registry: ${{ env.REGISTRY_DOCKERHUB }}
    username: ${{ secrets.DOCKERHUB_USERNAME }}
    password: ${{ secrets.DOCKERHUB_TOKEN }}
```

3. Update metadata to publish to both registries:

```yaml
- name: Extract metadata (tags, labels)
  id: meta
  uses: docker/metadata-action@v5
  with:
    images: |
      ${{ env.REGISTRY_GHCR }}/${{ github.repository_owner }}/kiisrv-${{ matrix.image }}
      ${{ env.REGISTRY_DOCKERHUB }}/${{ github.repository_owner }}/kiisrv-${{ matrix.image }}
```

### Build for Multiple Platforms

To support ARM64 (Raspberry Pi, Apple Silicon):

```yaml
- name: Build and push Docker image
  uses: docker/build-push-action@v5
  with:
    platforms: linux/amd64,linux/arm64  # Add ARM64
    # ... rest of config
```

**Warning:** ARM builds are slow in CI. Consider:
- Only building ARM on tagged releases
- Using self-hosted ARM runners
- Building selectively (main image only, not all controllers)

## Monitoring

### View Workflow Status

GitHub repo → Actions tab → "Build and Publish Docker Images"

### Check Published Packages

GitHub repo → Packages (right sidebar)

Or visit:
- `https://github.com/orgs/{org}/packages` (for orgs)
- `https://github.com/users/{user}/packages` (for personal)

### Download Statistics

GitHub Container Registry provides pull statistics in the package view.

## Troubleshooting

### Workflow Fails

**Check logs:**
- GitHub repo → Actions → Click the failed workflow → View logs

**Common issues:**
- Timeout (>6 hours) - Won't happen with this workflow
- Out of disk space - GitHub runners have ~14GB, should be enough
- Network issues fetching dependencies - Re-run workflow

### Images Not Found After Build

**Verify workflow completed successfully:**
- All jobs should show green checkmarks

**Check package visibility:**
- Public repositories → Packages are public by default
- Private repositories → May need to configure package visibility

**Check package exists:**
```bash
docker pull ghcr.io/{user}/kiisrv-kiisrv:containerize
```

### Permission Denied

**For public packages:**
- Should work without authentication

**For private packages:**
```bash
echo $GITHUB_TOKEN | docker login ghcr.io -u {username} --password-stdin
docker pull ghcr.io/{user}/kiisrv-kiisrv:latest
```

## Cost

**GitHub Container Registry:**
- ✅ **FREE** for public repositories
- ✅ Unlimited storage and bandwidth for public packages
- Private repos: 500MB free storage

**GitHub Actions:**
- ✅ **FREE** for public repositories (unlimited minutes)
- Private repos: 2,000 minutes/month free
- This workflow uses ~30-45 minutes per run initially

## Security Notes

1. **GITHUB_TOKEN**: Automatically provided by GitHub, scoped to the repository
2. **Package permissions**: Set in workflow (`packages: write`)
3. **Vulnerability scanning**: Consider adding Trivy or similar
4. **Image signing**: Consider using cosign for production

## Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [GitHub Container Registry](https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-container-registry)
- [Docker Build Push Action](https://github.com/docker/build-push-action)
- [Deployment Guide](docs/GITHUB_ACTIONS_DEPLOYMENT.md)

---

**Questions?** See [docs/GITHUB_ACTIONS_DEPLOYMENT.md](docs/GITHUB_ACTIONS_DEPLOYMENT.md) for detailed usage instructions.

