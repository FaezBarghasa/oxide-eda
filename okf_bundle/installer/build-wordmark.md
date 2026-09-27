---
okf_version: "0.2"
type: Module
title: build-wordmark
description: Rasterize the Oxide wordmark SVGs into PNGs at 1x / 2x / 3x DPI tiers.
resource: installer/build-wordmark.py
tags:
  - "lang:python"
  - "type:Module"
  - "module:installer"
  - "domain:build-wordmark.py"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:00Z"
concept_id: installer/build-wordmark
language: python
---

# build-wordmark

Rasterize the Oxide wordmark SVGs into PNGs at 1x / 2x / 3x DPI tiers.

## Docstring

Rasterize the Oxide wordmark SVGs into PNGs at 1x / 2x / 3x DPI tiers.

The app menu bar displays the wordmark at 96x31 logical pixels. On a
100%-scale monitor that's 96x31 device pixels; at 200% scale winit
reports a scale factor of 2.0 and the same logical box is 192x62 device
pixels. Rasterizing fresh PNGs for each tier lets us hand iced an asset
that is 1:1 with the target device-pixel count, which sidesteps resvg's
unhinted path-text blur we'd otherwise get when iced rasterizes the SVG
down to ~31 px tall.

Outputs (committed under assets/brand/generated/):
    wordmark-white-1x.png   96x31
    wordmark-white-2x.png  192x62
    wordmark-white-3x.png  288x93
    wordmark-black-1x.png   96x31
    wordmark-black-2x.png  192x62
    wordmark-black-3x.png  288x93

Requires: resvg_py (pip install resvg-py). resvg is the same rasterizer
Firefox / Servo use, so it handles the logo's linear gradients cleanly.

## Relationships

| Type | Target |
|------|--------|
| related | [main](/installer/build-wordmark/main.md) |
