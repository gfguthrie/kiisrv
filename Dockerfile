# Optimized Dockerfile using Debian Slim
# Expected size reduction: 40-50% compared to full Ubuntu

FROM debian:bookworm-slim AS base

ENV DEBIAN_FRONTEND=noninteractive

# Install locales (required for builds)
RUN apt-get update && \
    apt-get install -y --no-install-recommends locales && \
    rm -rf /var/lib/apt/lists/*

RUN echo "LANG=en_US.UTF-8" > /etc/locale.conf && \
    echo "en_US.UTF-8 UTF-8" >> /etc/locale.gen
ENV LANG=en_US.UTF-8 LC_ALL=en_US.UTF-8 LANGUAGE=en_US:en
RUN locale-gen

# Install ONLY essential packages for firmware builds
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    # Build system
    cmake ninja-build ccache universal-ctags \
    # ARM cross-compiler (the big one)
    gcc-arm-none-eabi binutils-arm-none-eabi libnewlib-arm-none-eabi \
    # Firmware tools
    dfu-util \
    # Python for KLL compiler
    python3 python3-pip python3-venv \
    # Version control
    git \
    # Archive tools
    zip \
    # CA certificates for HTTPS
    ca-certificates \
    # LSB tools (needed by CMake build system)
    lsb-release \
    # Hex utilities (needed for secure firmware generation)
    bsdextrautils \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean \
    && rm -rf /tmp/* /var/tmp/*

# Install Python packages (no cache, minimal deps)
RUN pip3 install --no-cache-dir --break-system-packages \
    pipenv \
    kll

# Setup ccache
RUN /usr/sbin/update-ccache-symlinks

# Pipenv config
ENV PIPENV_VENV_IN_PROJECT=1 \
    PIPENV_CUSTOM_VENV_NAME=.venv \
    PIP_NO_CACHE_DIR=1

# Setup scripts
ADD update_kll_cache.sh /usr/local/bin/update_kll_cache.sh
RUN chmod +x /usr/local/bin/update_kll_cache.sh

ADD build.sh /usr/local/bin/build.sh
RUN chmod +x /usr/local/bin/build.sh

# Controller-specific stage
FROM base AS controller

ARG TAG=master

# Clone with --depth 1 to save space (don't need full history)
RUN git clone --depth 1 https://github.com/kiibohd/controller.git -b $TAG

RUN echo $TAG > controller/TAG

# Setup pipenv environment
RUN cd controller/Keyboards && pipenv install --deploy

# Cache KLL layouts
RUN cd controller/Keyboards && pipenv run /usr/local/bin/update_kll_cache.sh

WORKDIR /controller/Keyboards
ENTRYPOINT ["/usr/local/bin/build.sh"]

# KiiSrv server stage - Rust builder
FROM rust:1.83-bookworm AS builder

WORKDIR /build

# Copy dependency files first for better layer caching
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY schema ./schema

# Build release binary
RUN cargo build --release

# KiiSrv server stage - Runtime
FROM debian:bookworm-slim AS kiisrv

# Install minimal runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    docker.io \
    git \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
# Note: We need to match the Docker socket GID which varies by system
# On Mac it's often 991, on Linux it's often 999 or similar
RUN useradd -m -u 1000 -s /bin/bash kiisrv && \
    groupadd -g 991 dockerhost || true && \
    usermod -aG docker,991 kiisrv

WORKDIR /app

# Copy binary from builder
COPY --from=builder /build/target/release/kiisrv /app/kiisrv

# Copy layouts and schema directories
COPY layouts ./layouts
COPY schema ./schema

# Create directories for runtime data
RUN mkdir -p tmp_builds tmp_config && \
    chown -R kiisrv:kiisrv /app

USER kiisrv

# Default environment variables (can be overridden)
ENV KIISRV_HOST=0.0.0.0 \
    KIISRV_PORT=3001

EXPOSE 3001

CMD ["/app/kiisrv"]

