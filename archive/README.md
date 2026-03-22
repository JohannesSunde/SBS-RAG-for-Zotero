# Archived Files

This directory contains deprecated files that are no longer actively used but are preserved for historical reference.

## Docker Files (`docker/`)

### `Dockerfile.build`
- **Deprecated**: Date unknown
- **Reason**: No longer referenced in build scripts or documentation
- **Replaced by**: `Dockerfile.linux-build` (for active Linux AppImage builds)

### `Dockerfile.pyinstaller-linux`
- **Deprecated**: v0.1.10 (December 2024)
- **Reason**: PyInstaller bundles exceeded GitHub's 2GB file size limit (created 2.5GB DEB packages)
- **Replaced by**: Linux now uses system Python + venv with auto-setup and repair logic
- **See**: CHANGELOG.md v0.1.10 for details on the reversion

## Scripts (`scripts/`)

### `build-linux-pyinstaller.sh`
- **Deprecated**: v0.1.10 (December 2024)
- **Reason**: No longer needed after reverting from PyInstaller to venv approach
- **Related**: Uses `Dockerfile.pyinstaller-linux` which is also deprecated

## Notes

- macOS and Windows still use PyInstaller bundles (self-contained)
- Linux uses a hybrid approach (system Python + venv) for smaller package sizes
- These files are kept for reference in case the approach changes again in the future
