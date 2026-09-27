---
okf_version: "0.2"
type: Class
title: ProjectionPassConfig
description: Configuration for the projection texture pass.
resource: crates/oxide-renderer/src/pcb3d/projection.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb3d/projection/ProjectionPassConfig
language: rust
---

# ProjectionPassConfig

Configuration for the projection texture pass.

## Signature

```rust
pub struct ProjectionPassConfig
```

## Decorators

- `derive(Clone, Copy, Debug, PartialEq)`

## Visibility

- `pub`

## Docstring

Configuration for the projection texture pass.

`footprint_bounds` defines the board-layer footprint extent in mm.
`uv_bounds` defines normalized UV coverage within that footprint; both
components must be in `[0.0, 1.0]` with `min < max`.
[derive(Clone, Copy, Debug, PartialEq)]

## Methods

- `footprint_bounds`
- `uv_bounds`
- `tile_columns`
- `fill_alpha`
- `stroke_width_mm`

## Source
Lines 44–50 in `crates/oxide-renderer/src/pcb3d/projection.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [projection](/crates/oxide-renderer/src/pcb3d/projection.md) |
