#!/bin/bash
# Build an AppImage from a compiled oxide binary. AppImage is a
# single-file portable Linux app — users download, chmod +x, run.
#
# Invocation:
#   installer/linux/build-appimage.sh <binary_path> <version> <arch>
#
#   <arch> — "x86_64" or "aarch64".
# Output: Oxide-<version>-<arch>.AppImage in the CWD.

set -euo pipefail

BINARY_PATH="${1:?usage: build-appimage.sh <binary> <version> <arch>}"
VERSION="${2:?missing version}"
ARCH="${3:?missing arch}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

WORK_DIR="$(mktemp -d)"
trap 'rm -rf "$WORK_DIR"' EXIT

APP_DIR="$WORK_DIR/Oxide.AppDir"
mkdir -p "$APP_DIR/usr/bin"
mkdir -p "$APP_DIR/usr/share/applications"
mkdir -p "$APP_DIR/usr/share/icons/hicolor/128x128/apps"
mkdir -p "$APP_DIR/usr/share/icons/hicolor/256x256/apps"
mkdir -p "$APP_DIR/usr/share/icons/hicolor/512x512/apps"

cp "$BINARY_PATH" "$APP_DIR/usr/bin/oxide"
chmod +x "$APP_DIR/usr/bin/oxide"

# AppRun — the entry point AppImage calls.
cat > "$APP_DIR/AppRun" <<'APPRUN_EOF'
#!/bin/bash
HERE="$(dirname "$(readlink -f "$0")")"
exec "$HERE/usr/bin/oxide" "$@"
APPRUN_EOF
chmod +x "$APP_DIR/AppRun"

# Top-level .desktop for AppImage tooling.
cat > "$APP_DIR/oxide.desktop" <<DESKTOP_EOF
[Desktop Entry]
Type=Application
Name=Oxide
Comment=AI-first EDA editor
Exec=oxide
Icon=oxide
Categories=Development;Electronics;Engineering;
Terminal=false
DESKTOP_EOF
cp "$APP_DIR/oxide.desktop" "$APP_DIR/usr/share/applications/oxide.desktop"

cp "$SCRIPT_DIR/oxide-256.png" "$APP_DIR/oxide.png"
cp "$SCRIPT_DIR/oxide-128.png" "$APP_DIR/usr/share/icons/hicolor/128x128/apps/oxide.png"
cp "$SCRIPT_DIR/oxide-256.png" "$APP_DIR/usr/share/icons/hicolor/256x256/apps/oxide.png"
cp "$SCRIPT_DIR/oxide-512.png" "$APP_DIR/usr/share/icons/hicolor/512x512/apps/oxide.png"

# appimagetool runtime (linuxdeploy in CI handles this; fall back to
# appimagetool AppImage if available locally).
if [[ -n "${APPIMAGETOOL:-}" ]]; then
    TOOL="$APPIMAGETOOL"
elif command -v appimagetool &>/dev/null; then
    TOOL="$(command -v appimagetool)"
elif [[ -x "$SCRIPT_DIR/appimagetool" ]]; then
    TOOL="$SCRIPT_DIR/appimagetool"
else
    echo "appimagetool not found. Set APPIMAGETOOL=/path/to/appimagetool or place it next to build-appimage.sh." >&2
    exit 1
fi

OUTPUT="Oxide-$VERSION-$ARCH.AppImage"
rm -f "$OUTPUT"

ARCH="$ARCH" "$TOOL" --no-appstream "$APP_DIR" "$OUTPUT"

chmod +x "$OUTPUT"
echo "Built $OUTPUT"
