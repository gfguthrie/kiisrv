# KiiSrv Documentation

This directory contains documentation for the kiisrv keyboard firmware build system.

## Documentation Files

### [MODERNIZATION.md](./MODERNIZATION.md) - START HERE
Complete overview of the 2025 modernization effort.
- **What changed:** Rust framework, dependencies, Docker setup
- **Docker architecture:** Multi-stage build explanation
- **GCC compatibility:** Why `-fcommon` is needed
- **Known issues:** Build warnings and test failures
- **Everything future maintainers need to know**

### [GCC_COMPATIBILITY.md](./GCC_COMPATIBILITY.md)
Technical deep-dive into the GCC 12+ compatibility fix.
- Detailed explanation of linker errors
- Why old firmware code breaks with modern GCC
- Alternative solutions considered
- Implementation details

## Quick Start

For setup and running instructions, see the [main README](../README.md).

## Key Takeaways

1. **Rust modernized:** Iron (2018) → Axum (2025), Rust 2018 → 2021
2. **Docker simplified:** Multi-stage Dockerfile, Debian slim base
3. **Critical fix needed:** `-fcommon` flag for GCC 12+ compatibility
4. **API unchanged:** Fully backward compatible

The system is production-ready!
