# kiisrv Documentation Index

Complete guide to all documentation files in this repository.

## 🚀 Getting Started

**New to kiisrv?** Start here:
1. **[README.md](README.md)** - Project overview and quick start
2. **[QUICK_START.md](QUICK_START.md)** - Choose your deployment method
3. **[docs/CONTAINERIZED_DEPLOYMENT.md](docs/CONTAINERIZED_DEPLOYMENT.md)** - Full deployment guide

## 📚 Main Documentation

### Deployment Guides

| File | Purpose | When to Use |
|------|---------|-------------|
| **[docs/GITHUB_ACTIONS_DEPLOYMENT.md](docs/GITHUB_ACTIONS_DEPLOYMENT.md)** | Using pre-built Docker images | Fastest deployment, no build needed |
| **[docs/GITHUB_ACTIONS_SETUP.md](docs/GITHUB_ACTIONS_SETUP.md)** | Setting up CI/CD pipeline | Initial repository setup |
| **[docs/CONTAINERIZED_DEPLOYMENT.md](docs/CONTAINERIZED_DEPLOYMENT.md)** | Production deployment with Docker | Deploying to VPS, self-hosting |
| **[QUICK_START.md](QUICK_START.md)** | Quick commands for all scenarios | Fast reference |
| **[docs/RUNNING_LOCALLY.md](docs/RUNNING_LOCALLY.md)** | Local development guide | Testing and development |

### Technical Documentation

| File | Purpose | Audience |
|------|---------|----------|
| **[docs/IMPLEMENTATION_NOTES.md](docs/IMPLEMENTATION_NOTES.md)** | Containerization technical details | Developers, troubleshooters |
| **[docs/MODERNIZATION.md](docs/MODERNIZATION.md)** | 2025 modernization overview | Maintainers, contributors |
| **[docs/GCC_COMPATIBILITY.md](docs/GCC_COMPATIBILITY.md)** | GCC 12+ compatibility fix | Technical deep-dive |

### Summary Documents

| File | Purpose | Audience |
|------|---------|----------|
| **[docs/CONTAINERIZATION_SUMMARY.md](docs/CONTAINERIZATION_SUMMARY.md)** | Containerization overview | Quick reference |
| **[docs/DEPLOYMENT_CONTEXT.md](docs/DEPLOYMENT_CONTEXT.md)** | Context for AI assistants | Copy-paste reference |

### Checklists and Procedures

| File | Purpose | When to Use |
|------|---------|-------------|
| **[docs/DEPLOYMENT_CHECKLIST.md](docs/DEPLOYMENT_CHECKLIST.md)** | Step-by-step deployment | Production rollout |

## 🎯 By Use Case

### "I want to deploy kiisrv to production"
1. [docs/GITHUB_ACTIONS_DEPLOYMENT.md](docs/GITHUB_ACTIONS_DEPLOYMENT.md) - Use pre-built images (fastest)
2. [docs/CONTAINERIZED_DEPLOYMENT.md](docs/CONTAINERIZED_DEPLOYMENT.md) - Or build locally
3. [docs/DEPLOYMENT_CHECKLIST.md](docs/DEPLOYMENT_CHECKLIST.md)

### "I want to self-host kiisrv"
1. [docs/GITHUB_ACTIONS_DEPLOYMENT.md](docs/GITHUB_ACTIONS_DEPLOYMENT.md) - Just pull and run (easiest)
2. [QUICK_START.md](QUICK_START.md) - Quick commands
3. [docs/CONTAINERIZED_DEPLOYMENT.md](docs/CONTAINERIZED_DEPLOYMENT.md) - Full guide

### "I'm developing/contributing to kiisrv"
1. [README.md](README.md) - Local Development section
2. [docs/RUNNING_LOCALLY.md](docs/RUNNING_LOCALLY.md)
3. [docs/MODERNIZATION.md](docs/MODERNIZATION.md)

### "I have an IPv6-only server"
1. [docs/CONTAINERIZED_DEPLOYMENT.md](docs/CONTAINERIZED_DEPLOYMENT.md) - Option 1
2. Build locally, transfer images

### "I want to understand how it works"
1. [docs/MODERNIZATION.md](docs/MODERNIZATION.md) - Architecture
2. [docs/IMPLEMENTATION_NOTES.md](docs/IMPLEMENTATION_NOTES.md) - Technical details
3. [docs/GCC_COMPATIBILITY.md](docs/GCC_COMPATIBILITY.md) - Firmware build fix

### "Something is broken"
1. [docs/CONTAINERIZED_DEPLOYMENT.md](docs/CONTAINERIZED_DEPLOYMENT.md) - Troubleshooting section
2. [docs/RUNNING_LOCALLY.md](docs/RUNNING_LOCALLY.md) - Troubleshooting section
3. [docs/IMPLEMENTATION_NOTES.md](docs/IMPLEMENTATION_NOTES.md) - Common issues

## 📋 Quick Reference

### Common Issues

**Empty versions endpoint:**
- See [docs/IMPLEMENTATION_NOTES.md](docs/IMPLEMENTATION_NOTES.md) - Challenge 5
- Fix: Ensure using `.execute_batch()` for database init

**Database files are directories:**
- See [docs/CONTAINERIZED_DEPLOYMENT.md](docs/CONTAINERIZED_DEPLOYMENT.md) - Troubleshooting
- Fix: Create with `touch` before `docker compose up`

**Docker permission denied:**
- See [docs/IMPLEMENTATION_NOTES.md](docs/IMPLEMENTATION_NOTES.md) - Challenge 3
- Fix: User needs group 991 access (Mac) or docker group (Linux)

**Firmware builds fail:**
- See [docs/IMPLEMENTATION_NOTES.md](docs/IMPLEMENTATION_NOTES.md) - Challenge 2
- Fix: Using `docker run` instead of `docker compose run`

### Key Commands

**Containerized:**
```bash
docker compose -f compose.prod.yaml build
docker compose -f compose.prod.yaml up        # Foreground
docker compose -f compose.prod.yaml up -d     # Background
docker compose -f compose.prod.yaml logs -f
docker compose -f compose.prod.yaml down
```

**Development:**
```bash
cargo run
cargo test
docker compose build controller-057
```

## 📖 Documentation Structure

```
kiisrv/
├── README.md                          # Main entry point
├── QUICK_START.md                     # Fast commands
├── DOCUMENTATION_INDEX.md             # This file - master index
└── docs/
    ├── README.md                      # Docs directory index
    ├── GITHUB_ACTIONS_DEPLOYMENT.md   # Pre-built images guide ⭐
    ├── GITHUB_ACTIONS_SETUP.md        # CI/CD setup instructions
    ├── CONTAINERIZED_DEPLOYMENT.md    # Primary deployment guide
    ├── DEPLOYMENT_CHECKLIST.md        # Production deployment checklist
    ├── IMPLEMENTATION_NOTES.md        # Technical deep-dive
    ├── RUNNING_LOCALLY.md             # Local development guide
    ├── CONTAINERIZATION_SUMMARY.md    # Containerization overview
    ├── DEPLOYMENT_CONTEXT.md          # AI assistant context
    ├── MODERNIZATION.md               # 2025 modernization story
    └── GCC_COMPATIBILITY.md           # GCC 12+ fix details
```

## 🔄 Documentation History

**Created November 2025** during containerization effort:
- docs/CONTAINERIZED_DEPLOYMENT.md - Primary deployment guide
- docs/DEPLOYMENT_CHECKLIST.md - Production deployment steps
- docs/IMPLEMENTATION_NOTES.md - Technical implementation deep-dive
- docs/CONTAINERIZATION_SUMMARY.md - Containerization overview
- docs/RUNNING_LOCALLY.md - Local development guide
- docs/GITHUB_ACTIONS_DEPLOYMENT.md - Pre-built images guide
- docs/GITHUB_ACTIONS_SETUP.md - CI/CD setup instructions
- QUICK_START.md - Fast deployment commands
- DOCUMENTATION_INDEX.md - This master index

**Updated for containerization:**
- README.md - Containerized deployment as primary method
- docs/README.md - Added containerization documentation
- docs/DEPLOYMENT_CONTEXT.md - Updated for containerized deployment

**Pre-existing (from 2025 modernization):**
- docs/MODERNIZATION.md - Rust/Docker modernization story
- docs/GCC_COMPATIBILITY.md - GCC 12+ compatibility fix

## 💡 Tips

- **Always create `config.db` and `stats.db` files before first Docker run**
- **Use compose.prod.yaml for containerized deployment** (production or development)
- **Use compose.yaml + cargo run for active Rust development** (faster iteration)
- **Both `cargo run` and containerized methods work identically**
- **Build images locally for IPv6-only servers**
- **Build only needed controllers**: `docker compose -f compose.prod.yaml build kiisrv controller-057`

## 🆘 Getting Help

1. Check the relevant guide above for your use case
2. Look in the Troubleshooting sections
3. Review [docs/IMPLEMENTATION_NOTES.md](docs/IMPLEMENTATION_NOTES.md) for technical issues
4. Check GitHub Issues
5. Review logs: `docker logs kiisrv` or `cargo run` output

