#!/bin/bash
# Helper script to pull pre-built images and tag them for local use
# kiisrv expects images named: kiisrv-controller-050, kiisrv-controller-057, etc.
# But registries store them with full paths: ghcr.io/owner/kiisrv-controller-050:tag
#
# Usage:
#   ./pull-and-tag.sh [compose-file] [registry-owner] [tag] [controllers]
#
# Examples:
#   ./pull-and-tag.sh                                    # Pull latest (057 only)
#   ./pull-and-tag.sh compose.ghcr.yaml kiibohd latest   # Pull latest (057 only)
#   ./pull-and-tag.sh compose.ghcr.yaml kiibohd latest all  # Pull all controllers
#   ./pull-and-tag.sh compose.ghcr.yaml kiibohd latest "050 057"  # Pull specific versions

# Note: We don't use 'set -e' because it's okay if some controller images don't exist

# Configuration
COMPOSE_FILE="${1:-compose.ghcr.yaml}"
REGISTRY_OWNER="${2:-kiibohd}"
TAG="${3:-latest}"
CONTROLLERS_ARG="${4:-057}"  # Default to latest only

# Parse controller versions
if [ "$CONTROLLERS_ARG" = "all" ]; then
    CONTROLLERS=("050" "054" "055" "056" "057")
else
    # Split space-separated list into array
    read -ra CONTROLLERS <<< "$CONTROLLERS_ARG"
fi

echo "====================================="
echo "Pull and Tag Script for kiisrv"
echo "====================================="
echo "Compose file: $COMPOSE_FILE"
echo "Registry owner: $REGISTRY_OWNER"
echo "Tag: $TAG"
echo "Controllers: ${CONTROLLERS[*]}"
echo ""

# Detect registry from compose file by looking at actual image declarations (not comments)
# Extract the kiisrv image line and check its format
KIISRV_IMAGE=$(grep "^[[:space:]]*image:" "$COMPOSE_FILE" | head -1 | sed 's/^[[:space:]]*image:[[:space:]]*//')

if echo "$KIISRV_IMAGE" | grep -q "ghcr.io"; then
    REGISTRY="ghcr.io"
    REGISTRY_PREFIX="ghcr.io/"
elif echo "$KIISRV_IMAGE" | grep -q "docker.io"; then
    REGISTRY="docker.io"
    REGISTRY_PREFIX="docker.io/"
else
    # No explicit registry means Docker Hub (default) - no prefix needed
    REGISTRY="docker.io"
    REGISTRY_PREFIX=""
fi

echo "Detected registry: $REGISTRY"
echo ""

# Pull all images
# Controller images have profiles in compose, so we pull them directly with docker
echo "Step 1: Pulling images from registry..."
echo "  Pulling main kiisrv image..."
docker compose -f "$COMPOSE_FILE" pull kiisrv

echo "  Pulling controller images..."
# Pull each controller directly since they have profile: manual in compose
for ctrl in "${CONTROLLERS[@]}"; do
    IMAGE="${REGISTRY_PREFIX}${REGISTRY_OWNER}/kiisrv-controller-$ctrl:$TAG"
    echo "    - $IMAGE"
    if docker pull "$IMAGE" 2>/dev/null; then
        echo "      ✓ Pulled successfully"
    else
        echo "      ⚠ Not available in registry (skipping)"
    fi
done

echo ""
echo "Step 2: Tagging images for local use..."

for ctrl in "${CONTROLLERS[@]}"; do
    SOURCE_IMAGE="${REGISTRY_PREFIX}${REGISTRY_OWNER}/kiisrv-controller-$ctrl:$TAG"
    TARGET_IMAGE="kiisrv-controller-$ctrl:latest"
    
    # Check if source image exists
    if docker image inspect "$SOURCE_IMAGE" >/dev/null 2>&1; then
        echo "  Tagging: $SOURCE_IMAGE -> $TARGET_IMAGE"
        docker tag "$SOURCE_IMAGE" "$TARGET_IMAGE"
    else
        echo "  WARNING: Image not found: $SOURCE_IMAGE (skipping)"
    fi
done

# Tag the main kiisrv image
KIISRV_SOURCE="${REGISTRY_PREFIX}${REGISTRY_OWNER}/kiisrv-kiisrv:$TAG"
KIISRV_TARGET="kiisrv-server:latest"

if docker image inspect "$KIISRV_SOURCE" >/dev/null 2>&1; then
    echo "  Tagging: $KIISRV_SOURCE -> $KIISRV_TARGET"
    docker tag "$KIISRV_SOURCE" "$KIISRV_TARGET"
fi

echo ""
echo "Step 3: Verifying tagged images..."
# Show locally tagged images (without registry prefix)
docker images | grep -E "^kiisrv-(controller|server)"

echo ""
echo "====================================="
echo "✅ Done! Images are ready to use."
echo "====================================="
echo ""
echo "Start kiisrv with:"
echo "  docker compose -f $COMPOSE_FILE up -d"


