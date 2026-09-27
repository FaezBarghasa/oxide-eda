---
okf_version: "0.2"
type: Function
title: export_dpi
description: "DPI written to `PdfOptions.dpi` at export time. Vector content"
resource: crates/oxide-app/src/app/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/state/mod/export_dpi
language: rust
---

# export_dpi

DPI written to `PdfOptions.dpi` at export time. Vector content

## Signature

```rust
impl PdfQuality { pub fn export_dpi(self) -> f32 }
```

## Visibility

- `pub`

## Docstring

DPI written to `PdfOptions.dpi` at export time. Vector content
ignores this; future raster fallbacks (embedded images,
rasterised symbol bodies) honour the verbatim picker label.

## Source
Lines 555–561 in `crates/oxide-app/src/app/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/state/mod.md) |
