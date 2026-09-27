---
okf_version: "0.2"
type: Class
title: PdfQuality
description: Output PDF resolution preset — Altium parity. Drives the Quality
resource: crates/oxide-app/src/app/state/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/state/mod/PdfQuality
language: rust
---

# PdfQuality

Output PDF resolution preset — Altium parity. Drives the Quality

## Signature

```rust
pub enum PdfQuality
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Output PDF resolution preset — Altium parity. Drives the Quality
dropdown in the Settings tab. The export pipeline is vector-only
today, so DPI only affects the *preview* rasterisation: a higher
preset gives a sharper preview when you zoom in. The mapped DPI
for the export-side `PdfOptions.dpi` is the picker label (72/300/
600) so future raster fallbacks (embedded images) get the user
intent verbatim.
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Source
Lines 534–538 in `crates/oxide-app/src/app/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/state/mod.md) |
