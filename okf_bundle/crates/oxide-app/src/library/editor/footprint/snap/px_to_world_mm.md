---
okf_version: "0.2"
type: Function
title: px_to_world_mm
description: Convert a screen-pixel radius to world-mm at the current camera
resource: crates/oxide-app/src/library/editor/footprint/snap.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/snap/px_to_world_mm
language: rust
---

# px_to_world_mm

Convert a screen-pixel radius to world-mm at the current camera

## Signature

```rust
pub fn px_to_world_mm(px: f32, scale_px_per_mm: f32) -> f64
```

## Visibility

- `pub`

## Docstring

Convert a screen-pixel radius to world-mm at the current camera
scale. Callers pass the canvas's `scale` (px/mm) and we return
the equivalent world-mm radius for distance comparisons.

## Source
Lines 94–96 in `crates/oxide-app/src/library/editor/footprint/snap.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [snap](/crates/oxide-app/src/library/editor/footprint/snap.md) |
