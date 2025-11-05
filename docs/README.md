# KiiSrv Documentation

This directory contains documentation for the kiisrv keyboard firmware build system.

## Documentation Files

### [CONTAINERIZED_DEPLOYMENT.md](./CONTAINERIZED_DEPLOYMENT.md) - START HERE FOR DEPLOYMENT
Complete guide for deploying containerized kiisrv.
- **Build locally, deploy anywhere** (solves IPv6 issues)
- **Self-hosting** instructions for end users
- **Production setup** with Nginx and SSL
- **Troubleshooting** common issues

### [IMPLEMENTATION_NOTES.md](./IMPLEMENTATION_NOTES.md) - Technical Details
Deep dive into containerization implementation.
- **Code changes** required for containerization
- **Challenge-solution** breakdown
- **Docker socket** pattern explanation
- **Backward compatibility** with cargo run

### [MODERNIZATION.md](./MODERNIZATION.md) - Modernization Overview
Complete overview of the 2025 modernization effort.
- **What changed:** Rust framework, dependencies, Docker setup
- **Docker architecture:** Multi-stage build explanation
- **GCC compatibility:** Why `-fcommon` is needed
- **Known issues:** Build warnings and test failures

### [GCC_COMPATIBILITY.md](./GCC_COMPATIBILITY.md) - GCC Fix Details
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
