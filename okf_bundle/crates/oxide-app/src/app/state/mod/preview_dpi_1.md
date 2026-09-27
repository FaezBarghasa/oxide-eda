---
okf_version: "0.2"
type: Function
title: preview_dpi
description: DPI used to rasterise the on-screen preview. Capped well below
resource: crates/oxide-app/src/app/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/state/mod/preview_dpi_1
language: rust
---

# preview_dpi

DPI used to rasterise the on-screen preview. Capped well below

## Signature

```rust
pub fn preview_dpi(self) -> f64
```

## Visibility

- `pub`

## Docstring

DPI used to rasterise the on-screen preview. Capped well below
the export label so an A4 page doesn't blow up to ~35 MB of
RGBA at 600 DPI.

## Source
Lines 544–550 in `crates/oxide-app/src/app/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/state/mod.md) |
