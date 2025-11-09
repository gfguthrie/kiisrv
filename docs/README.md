# KiiSrv Documentation

This directory contains documentation for the kiisrv keyboard firmware build system.

## Documentation Files

### [GITHUB_ACTIONS_DEPLOYMENT.md](./GITHUB_ACTIONS_DEPLOYMENT.md) - PRE-BUILT IMAGES ⭐
Guide for using pre-built Docker images from GitHub Container Registry.
- **No building required** - just pull and run
- **2-3 minute deployment** vs 20-30 minutes
- **Self-hosting** made incredibly easy
- **IPv6 compatible** - works everywhere

### [GITHUB_ACTIONS_SETUP.md](./GITHUB_ACTIONS_SETUP.md) - CI/CD SETUP
Instructions for setting up automated builds and publishing.
- **GitHub Actions workflow** configuration
- **Image publishing** to GitHub Container Registry
- **Testing and verification** steps
- **Customization options**

### [CONTAINERIZED_DEPLOYMENT.md](./CONTAINERIZED_DEPLOYMENT.md) - FULL DEPLOYMENT GUIDE
Complete guide for deploying containerized kiisrv.
- **Build locally, deploy anywhere** (solves IPv6 issues)
- **Self-hosting** instructions for end users
- **Production setup** with Nginx and SSL
- **Troubleshooting** common issues

### [DEPLOYMENT_CHECKLIST.md](./DEPLOYMENT_CHECKLIST.md) - PRODUCTION CHECKLIST
Step-by-step checklist for production deployments.
- **Pre-deployment** verification
- **Server setup** procedures
- **Network configuration**
- **Post-deployment** validation

### [IMPLEMENTATION_NOTES.md](./IMPLEMENTATION_NOTES.md) - TECHNICAL DETAILS
Deep dive into containerization implementation.
- **Code changes** required for containerization
- **Challenge-solution** breakdown
- **Docker socket** pattern explanation
- **Backward compatibility** with cargo run

### [RUNNING_LOCALLY.md](./RUNNING_LOCALLY.md) - LOCAL DEVELOPMENT
Quick reference for running kiisrv locally during development.
- **Foreground vs background** modes
- **Building specific controllers**
- **Development workflow**
- **Testing procedures**

### [MODERNIZATION.md](./MODERNIZATION.md) - MODERNIZATION OVERVIEW
Complete overview of the 2025 modernization effort.
- **What changed:** Rust framework, dependencies, Docker setup
- **Docker architecture:** Multi-stage build explanation
- **GCC compatibility:** Why `-fcommon` is needed
- **Known issues:** Build warnings and test failures

### [GCC_COMPATIBILITY.md](./GCC_COMPATIBILITY.md) - GCC FIX DETAILS
Technical deep-dive into the GCC 12+ compatibility fix.
- Detailed explanation of linker errors
- Why old firmware code breaks with modern GCC
- Alternative solutions considered
- Implementation details

## Quick Start

For setup and running instructions, see the [main README](../README.md).

## Key Takeaways

1. **Fully containerized:** Both server and build environments now run in Docker
2. **Rust modernized:** Iron (2018) → Axum (2025), Rust 2018 → 2021
3. **Docker simplified:** Multi-stage Dockerfile, Debian slim base
4. **Critical fix needed:** `-fcommon` flag for GCC 12+ compatibility
5. **API unchanged:** Fully backward compatible
6. **Dual deployment:** Works with both `cargo run` and `docker compose up`

The system is production-ready and solves IPv6-only server deployment!
