# Release Process

This document describes how to create and publish releases for the Task Manager API.

## Automated Releases

Releases are automatically built and published when you push a version tag to GitHub.

### Creating a Release

1. **Update version in Cargo.toml** (optional, but recommended):
   ```toml
   [package]
   version = "1.0.0"
   ```

2. **Commit changes**:
   ```bash
   git add Cargo.toml Cargo.lock
   git commit -m "Bump version to 1.0.0"
   ```

3. **Create and push a version tag**:
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

4. **GitHub Actions will automatically**:
   - Build binaries for multiple platforms
   - Create a GitHub Release
   - Upload all binaries to the release
   - Generate release notes

## Supported Platforms

The release workflow builds binaries for:

| Platform | Target | Binary Name | Archive Format |
|----------|--------|-------------|----------------|
| **Linux (glibc)** | x86_64-unknown-linux-gnu | task-manager | .tar.gz |
| **Linux (musl)** | x86_64-unknown-linux-musl | task-manager | .tar.gz |
| **Windows** | x86_64-pc-windows-msvc | task-manager.exe | .zip |

### Platform Notes

- **Linux (glibc)**: Standard Linux build, requires glibc (most common)
- **Linux (musl)**: Statically linked, works on any Linux without glibc dependencies (Alpine, etc.)
- **Windows**: Requires Visual C++ Redistributable (usually pre-installed)

## Release Workflow Details

The release workflow ([.github/workflows/release.yml](/.github/workflows/release.yml)) performs:

1. **Create Release**: Creates a GitHub release with auto-generated notes
2. **Build Binaries**: Compiles optimized release binaries for each platform
3. **Package**: Creates compressed archives (tar.gz for Linux, zip for Windows)
4. **Upload**: Attaches all binaries to the GitHub release
5. **Artifacts**: Keeps build artifacts for 7 days for debugging

## Version Numbering

Follow [Semantic Versioning](https://semver.org/):

- **v1.0.0**: Major version (breaking changes)
- **v1.1.0**: Minor version (new features, backward compatible)
- **v1.1.1**: Patch version (bug fixes)

## Manual Testing Before Release

Before creating a release tag, ensure:

1. ✅ All tests pass: `make test`
2. ✅ CI pipeline is green on main branch
3. ✅ Docker build works: `make test-docker`
4. ✅ Local release build works: `cargo build --release`
5. ✅ Security audit passes: `cargo audit --ignore RUSTSEC-2023-0071`

## Rollback a Release

If you need to remove a bad release:

```bash
# Delete the tag locally
git tag -d v1.0.0

# Delete the tag on GitHub
git push origin :refs/tags/v1.0.0

# Delete the GitHub Release manually from the GitHub UI
```

## Pre-releases

To create a pre-release (beta, rc, etc.):

```bash
git tag v1.0.0-beta.1
git push origin v1.0.0-beta.1
```

Mark it as a pre-release in the GitHub UI after it's created.

## Download and Installation

Users can download releases from the [GitHub Releases page](../../releases).

### Linux Installation

```bash
# Download and extract
wget https://github.com/USERNAME/task-manager/releases/download/v1.0.0/task-manager-linux-x86_64.tar.gz
tar xzf task-manager-linux-x86_64.tar.gz

# Make executable and run
chmod +x task-manager
./task-manager
```

### Windows Installation

1. Download `task-manager-windows-x86_64.zip` from releases
2. Extract the zip file
3. Run `task-manager.exe`

### Environment Configuration

All binaries require these environment variables:

```bash
export DATABASE_URL="postgres://user:password@localhost:5432/task_manager"
export JWT_SECRET="your_secret_key_here"
```

Or use a `.env` file in the same directory as the binary.

## Troubleshooting

### Build Fails on Windows

- Ensure OpenSSL is installed (GitHub Actions installs it automatically)
- Check that Visual Studio Build Tools are available

### Binary Won't Run on Linux

- Try the musl build if you get glibc version errors
- Check that you have executable permissions: `chmod +x task-manager`

### Missing Environment Variables

```
ERROR: DATABASE_URL must be set
```

Solution: Create a `.env` file or set environment variables before running.

## CI/CD Pipeline

The project has two workflows:

1. **CI** ([.github/workflows/ci.yml](/.github/workflows/ci.yml))
   - Runs on every push/PR
   - Tests, linting, security audit
   - Docker build verification

2. **Release** ([.github/workflows/release.yml](/.github/workflows/release.yml))
   - Runs only on version tags (v*.*.*)
   - Builds multi-platform binaries
   - Creates GitHub releases
