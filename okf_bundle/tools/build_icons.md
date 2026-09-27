---
okf_version: "0.2"
type: Module
title: build_icons
description: Pure-Python fallback for installer/build-icons.sh.
resource: tools/build_icons.py
tags:
  - "lang:python"
  - "type:Module"
  - "module:tools"
  - "domain:build_icons.py"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:00Z"
concept_id: tools/build_icons
language: python
---

# build_icons

Pure-Python fallback for installer/build-icons.sh.

## Docstring

Pure-Python fallback for installer/build-icons.sh.

Renders oxide-mark.svg to platform icon bitmaps using resvg_py (no native
Cairo/ImageMagick/Inkscape dependency). Produces the same outputs as the
bash script:
    installer/windows/oxide.ico
    installer/macos/Oxide.icns
    installer/linux/oxide-{128,256,512}.png
    crates/oxide-app/assets/brand/generated/oxide-{256,512}.png
    crates/oxide-app/assets/brand/generated/oxide.ico

Usage:
    py tools/build_icons.py

Requires: pip install resvg-py pillow

## Relationships

| Type | Target |
|------|--------|
| related | [render](/tools/build_icons/render.md) |
| related | [build_ico](/tools/build_icons/build_ico.md) |
| related | [build_icns](/tools/build_icons/build_icns.md) |
| related | [main](/tools/build_icons/main.md) |
