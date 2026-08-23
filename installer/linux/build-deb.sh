#!/bin/bash
# Build a Debian .deb package from a compiled oxide binary.
# Produces: oxide_<version>_<arch>.deb (Debian naming convention).
#
# Invocation:
#   installer/linux/build-deb.sh <binary_path> <version> <arch>
#
#   <arch> — "amd64" (x86_64) or "arm64" (aarch64) in Debian terminology.

set -euo pipefail

BINARY_PATH="${1:?usage: build-deb.sh <binary> <version> <arch>}"
VERSION="${2:?missing version}"
ARCH="${3:?missing arch}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

WORK_DIR="$(mktemp -d)"
trap 'rm -rf "$WORK_DIR"' EXIT

PKG_DIR="$WORK_DIR/oxide_${VERSION}_${ARCH}"
mkdir -p "$PKG_DIR/DEBIAN"
mkdir -p "$PKG_DIR/usr/bin"
mkdir -p "$PKG_DIR/usr/share/applications"
mkdir -p "$PKG_DIR/usr/share/icons/hicolor/128x128/apps"
mkdir -p "$PKG_DIR/usr/share/icons/hicolor/256x256/apps"
mkdir -p "$PKG_DIR/usr/share/icons/hicolor/512x512/apps"
mkdir -p "$PKG_DIR/usr/share/icons/hicolor/scalable/apps"
mkdir -p "$PKG_DIR/usr/share/icons/hicolor/scalable/mimetypes"
mkdir -p "$PKG_DIR/usr/share/mime/packages"
mkdir -p "$PKG_DIR/usr/share/doc/oxide"

cp "$BINARY_PATH" "$PKG_DIR/usr/bin/oxide"
chmod 755 "$PKG_DIR/usr/bin/oxide"

# Oxide application icons for desktop launchers. The desktop entry
# below uses Icon=oxide, so hicolor app icons must be installed under
# the matching basename at each available size.
cp "$SCRIPT_DIR/oxide-128.png" "$PKG_DIR/usr/share/icons/hicolor/128x128/apps/oxide.png"
cp "$SCRIPT_DIR/oxide-256.png" "$PKG_DIR/usr/share/icons/hicolor/256x256/apps/oxide.png"
cp "$SCRIPT_DIR/oxide-512.png" "$PKG_DIR/usr/share/icons/hicolor/512x512/apps/oxide.png"

# Oxide file-format icons (SVG) — Linux uses the SVG source
# directly, scaled by the desktop environment. Names follow the
# freedesktop.org MIME-icon convention
# `application-vnd.alpcaner.oxide.<ext>.svg` so the
# hicolor-scalable theme pairs them with the MIME types declared
# below.
REPO_ROOT_FOR_ICONS="$(cd "$SCRIPT_DIR/../.." && pwd)"
FILE_ICON_DIR="$REPO_ROOT_FOR_ICONS/crates/oxide-app/assets/icons/files"
for ext in snxprj snxsch snxpcb snxfpt snxsim snxlib snxsym snxpkg snxmat snxcfg snxmod; do
  src="$FILE_ICON_DIR/$ext.svg"
  if [[ -f "$src" ]]; then
    cp "$src" "$PKG_DIR/usr/share/icons/hicolor/scalable/mimetypes/application-vnd.alpcaner.oxide.$ext.svg"
  fi
done

# freedesktop.org MIME XML — one glob per Oxide extension.
cat > "$PKG_DIR/usr/share/mime/packages/oxide.xml" <<MIME_EOF
<?xml version="1.0" encoding="UTF-8"?>
<mime-info xmlns="http://www.freedesktop.org/standards/shared-mime-info">
  <mime-type type="application/vnd.alpcaner.oxide.snxprj">
    <comment>Oxide Project</comment>
    <glob pattern="*.snxprj"/>
    <icon name="application-vnd.alpcaner.oxide.snxprj"/>
  </mime-type>
  <mime-type type="application/vnd.alpcaner.oxide.snxsch">
    <comment>Oxide Schematic</comment>
    <glob pattern="*.snxsch"/>
    <icon name="application-vnd.alpcaner.oxide.snxsch"/>
  </mime-type>
  <mime-type type="application/vnd.alpcaner.oxide.snxpcb">
    <comment>Oxide PCB</comment>
    <glob pattern="*.snxpcb"/>
    <icon name="application-vnd.alpcaner.oxide.snxpcb"/>
  </mime-type>
  <mime-type type="application/vnd.alpcaner.oxide.snxfpt">
    <comment>Oxide Footprint</comment>
    <glob pattern="*.snxfpt"/>
    <icon name="application-vnd.alpcaner.oxide.snxfpt"/>
  </mime-type>
  <mime-type type="application/vnd.alpcaner.oxide.snxsim">
    <comment>Oxide Simulation</comment>
    <glob pattern="*.snxsim"/>
    <icon name="application-vnd.alpcaner.oxide.snxsim"/>
  </mime-type>
  <mime-type type="application/vnd.alpcaner.oxide.snxlib">
    <comment>Oxide Library</comment>
    <glob pattern="*.snxlib"/>
    <icon name="application-vnd.alpcaner.oxide.snxlib"/>
  </mime-type>
  <mime-type type="application/vnd.alpcaner.oxide.snxsym">
    <comment>Oxide Symbol</comment>
    <glob pattern="*.snxsym"/>
    <icon name="application-vnd.alpcaner.oxide.snxsym"/>
  </mime-type>
  <mime-type type="application/vnd.alpcaner.oxide.snxpkg">
    <comment>Oxide Package</comment>
    <glob pattern="*.snxpkg"/>
    <icon name="application-vnd.alpcaner.oxide.snxpkg"/>
  </mime-type>
  <mime-type type="application/vnd.alpcaner.oxide.snxmat">
    <comment>Oxide PCB Material</comment>
    <glob pattern="*.snxmat"/>
    <icon name="application-vnd.alpcaner.oxide.snxmat"/>
  </mime-type>
  <mime-type type="application/vnd.alpcaner.oxide.snxcfg">
    <comment>Oxide Config</comment>
    <glob pattern="*.snxcfg"/>
    <icon name="application-vnd.alpcaner.oxide.snxcfg"/>
  </mime-type>
  <mime-type type="application/vnd.alpcaner.oxide.snxmod">
    <comment>Oxide SPICE Model</comment>
    <glob pattern="*.snxmod"/>
    <icon name="application-vnd.alpcaner.oxide.snxmod"/>
  </mime-type>
</mime-info>
MIME_EOF

# Control file — Installed-Size is approx (binary + assets); dpkg-deb
# computes the actual compressed size when it builds.
INSTALLED_SIZE=$(du -sk "$PKG_DIR/usr" | cut -f1)
cat > "$PKG_DIR/DEBIAN/control" <<CONTROL_EOF
Package: oxide
Version: $VERSION
Section: electronics
Priority: optional
Architecture: $ARCH
Depends: libc6, libgcc-s1, libxkbcommon0, libxkbcommon-x11-0, libwayland-client0, libx11-6, libxcursor1, libxrandr2, libxi6, libxinerama1, libvulkan1
Installed-Size: $INSTALLED_SIZE
Maintainer: alpCaner <alpcaner92@gmail.com>
Homepage: https://github.com/alplabai/oxide
Description: AI-first EDA editor
 Oxide is an electronics design automation editor with an
 Altium-inspired interaction layer and native `.snx***` file
 formats.
CONTROL_EOF

# .desktop entry for menu integration. MimeType covers the Oxide
# native extensions declared in `usr/share/mime/packages/oxide.xml`.
cat > "$PKG_DIR/usr/share/applications/oxide.desktop" <<DESKTOP_EOF
[Desktop Entry]
Type=Application
Name=Oxide
Comment=AI-first EDA editor
Exec=/usr/bin/oxide %F
Icon=oxide
Terminal=false
Categories=Development;Electronics;Engineering;
MimeType=application/vnd.alpcaner.oxide.snxprj;application/vnd.alpcaner.oxide.snxsch;application/vnd.alpcaner.oxide.snxpcb;application/vnd.alpcaner.oxide.snxfpt;application/vnd.alpcaner.oxide.snxsim;application/vnd.alpcaner.oxide.snxlib;application/vnd.alpcaner.oxide.snxsym;application/vnd.alpcaner.oxide.snxpkg;application/vnd.alpcaner.oxide.snxmat;application/vnd.alpcaner.oxide.snxcfg;application/vnd.alpcaner.oxide.snxmod;
StartupNotify=true
DESKTOP_EOF

# postinst / postrm refresh the MIME + icon caches so Nautilus and
# friends pick up the new .snx*** types without a logout. Both
# scripts tolerate missing tools (update-mime-database, update-desktop-database,
# gtk-update-icon-cache) silently — user may be on a minimal install.
cat > "$PKG_DIR/DEBIAN/postinst" <<'POSTINST_EOF'
#!/bin/sh
set -e
if command -v update-mime-database >/dev/null 2>&1; then
  update-mime-database /usr/share/mime >/dev/null 2>&1 || true
fi
if command -v update-desktop-database >/dev/null 2>&1; then
  update-desktop-database /usr/share/applications >/dev/null 2>&1 || true
fi
if command -v gtk-update-icon-cache >/dev/null 2>&1; then
  gtk-update-icon-cache -q -t /usr/share/icons/hicolor >/dev/null 2>&1 || true
fi
exit 0
POSTINST_EOF
chmod 755 "$PKG_DIR/DEBIAN/postinst"

cat > "$PKG_DIR/DEBIAN/postrm" <<'POSTRM_EOF'
#!/bin/sh
set -e
if command -v update-mime-database >/dev/null 2>&1; then
  update-mime-database /usr/share/mime >/dev/null 2>&1 || true
fi
if command -v update-desktop-database >/dev/null 2>&1; then
  update-desktop-database /usr/share/applications >/dev/null 2>&1 || true
fi
if command -v gtk-update-icon-cache >/dev/null 2>&1; then
  gtk-update-icon-cache -q -t /usr/share/icons/hicolor >/dev/null 2>&1 || true
fi
exit 0
POSTRM_EOF
chmod 755 "$PKG_DIR/DEBIAN/postrm"

# README copied into the docs dir — Debian policy expects at least a
# changelog, but a short README keeps the lintian warning small.
cat > "$PKG_DIR/usr/share/doc/oxide/README.Debian" <<DOC_EOF
Oxide $VERSION
---------------
Installed by oxide_${VERSION}_${ARCH}.deb.
Report issues at https://github.com/alplabai/oxide/issues.
DOC_EOF

OUTPUT="oxide_${VERSION}_${ARCH}.deb"
rm -f "$OUTPUT"

# --root-owner-group emits root:root for every file, which lintian requires.
dpkg-deb --root-owner-group --build "$PKG_DIR" "$OUTPUT"

echo "Built $OUTPUT"
