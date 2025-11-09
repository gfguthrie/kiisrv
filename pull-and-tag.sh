#!/bin/bash
# Helper script to pull pre-built images and tag them for local use
# kiisrv expects images named: kiisrv-controller-050, kiisrv-controller-057, etc.
# But registries store them with full paths: ghcr.io/owner/kiisrv-controller-050:tag

set -e

# Configuration
COMPOSE_FILE="${1:-compose.ghcr.yaml}"
REGISTRY_OWNER="${2:-kiibohd}"
TAG="${3:-latest}"

echo "====================================="
echo "Pull and Tag Script for kiisrv"
echo "====================================="
echo "Compose file: $COMPOSE_FILE"
echo "Registry owner: $REGISTRY_OWNER"
echo "Tag: $TAG"
echo ""

# Detect registry from compose file
if grep -q "ghcr.io" "$COMPOSE_FILE"; then
    REGISTRY="ghcr.io"
elif grep -q "docker.io" "$COMPOSE_FILE" || grep -q "index.docker.io" "$COMPOSE_FILE"; then
    REGISTRY="docker.io"
else
    echo "Could not detect registry from $COMPOSE_FILE"
    exit 1
fi

echo "Detected registry: $REGISTRY"
echo ""

# Pull all images using docker compose
echo "Step 1: Pulling images from registry..."
docker compose -f "$COMPOSE_FILE" pull

echo ""
echo "Step 2: Tagging images for local use..."

# Controller versions to tag
CONTROLLERS=("050" "054" "055" "056" "057")

for ctrl in "${CONTROLLERS[@]}"; do
    SOURCE_IMAGE="$REGISTRY/$REGISTRY_OWNER/kiisrv-controller-$ctrl:$TAG"
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
KIISRV_SOURCE="$REGISTRY/$REGISTRY_OWNER/kiisrv-kiisrv:$TAG"
KIISRV_TARGET="kiisrv-server:latest"

if docker image inspect "$KIISRV_SOURCE" >/dev/null 2>&1; then
    echo "  Tagging: $KIISRV_SOURCE -> $KIISRV_TARGET"
    docker tag "$KIISRV_SOURCE" "$KIISRV_TARGET"
fi

echo ""
echo "Step 3: Verifying tagged images..."
docker images | grep -E "kiisrv-(controller|server)" | grep -v "$REGISTRY"

echo ""
echo "====================================="
echo "✅ Done! Images are ready to use."
echo "====================================="
echo ""
echo "Start kiisrv with:"
echo "  docker compose -f $COMPOSE_FILE up -d"


