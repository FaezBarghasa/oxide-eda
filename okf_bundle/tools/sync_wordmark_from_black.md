---
okf_version: "0.2"
type: Module
title: sync_wordmark_from_black
description: "Mirror the <path id=\"text11\"> 'd' attribute from oxide-logo-black.svg into"
resource: tools/sync_wordmark_from_black.py
tags:
  - "lang:python"
  - "type:Module"
  - "module:tools"
  - "domain:sync_wordmark_from_black.py"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:00Z"
concept_id: tools/sync_wordmark_from_black
language: python
---

# sync_wordmark_from_black

Mirror the <path id="text11"> 'd' attribute from oxide-logo-black.svg into

## Docstring

Mirror the <path id="text11"> 'd' attribute from oxide-logo-black.svg into
oxide-logo.svg and oxide-logo-white.svg, preserving each file's own fill.

The black variant is the user's manually-tuned reference (mark-to-wordmark gap).
Run this after adjusting the black SVG to keep all three logos in sync.

Usage:
    py tools/sync_wordmark_from_black.py

## Relationships

| Type | Target |
|------|--------|
| related | [extract_text11_d](/tools/sync_wordmark_from_black/extract_text11_d.md) |
| related | [replace_text11_d](/tools/sync_wordmark_from_black/replace_text11_d.md) |
| related | [main](/tools/sync_wordmark_from_black/main.md) |
