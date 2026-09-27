---
okf_version: "0.2"
type: Module
title: regen_wordmark_path
description: "Regenerate the 'oxide' wordmark path in brand SVGs from Panton-Bold.ttf."
resource: tools/regen_wordmark_path.py
tags:
  - "lang:python"
  - "type:Module"
  - "module:tools"
  - "domain:regen_wordmark_path.py"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:00Z"
concept_id: tools/regen_wordmark_path
language: python
---

# regen_wordmark_path

Regenerate the 'oxide' wordmark path in brand SVGs from Panton-Bold.ttf.

## Docstring

Regenerate the 'oxide' wordmark path in brand SVGs from Panton-Bold.ttf.

The three logo SVGs (oxide-logo.svg, oxide-logo-white.svg, oxide-logo-black.svg)
embed a <path id="text11"> whose 'd' attribute is the outlined 'oxide' wordmark.
When that path was originally exported from Inkscape, Panton was not installed
on the conversion machine, so the outlines came from a fallback font. This script
rebuilds the 'd' attribute by reading Panton-Bold.ttf directly with fontTools,
guaranteeing correct glyph geometry regardless of installed system fonts.

Usage:
    py tools/regen_wordmark_path.py

Geometry (matches existing wordmark placement):
    font-size    : 300 px
    letter-spacing: -7 px (per existing style)
    baseline y   : 330
    left x       : 496.75 (origin of the first glyph's bbox)
    fill         : preserved from existing style attribute

## Relationships

| Type | Target |
|------|--------|
| related | [build_wordmark_path_d](/tools/regen_wordmark_path/build_wordmark_path_d.md) |
| related | [replace_path_d](/tools/regen_wordmark_path/replace_path_d.md) |
| related | [main](/tools/regen_wordmark_path/main.md) |
