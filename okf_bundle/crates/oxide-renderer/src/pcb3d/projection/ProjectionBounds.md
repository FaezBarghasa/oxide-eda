---
okf_version: "0.2"
type: Class
title: ProjectionBounds
description: Footprint-space or UV-space rectangular bounds used by the projection pass.
resource: crates/oxide-renderer/src/pcb3d/projection.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb3d/projection/ProjectionBounds
language: rust
---

# ProjectionBounds

Footprint-space or UV-space rectangular bounds used by the projection pass.

## Signature

```rust
pub struct ProjectionBounds
```

## Decorators

- `derive(Clone, Copy, Debug, PartialEq)`

## Visibility

- `pub`

## Docstring

Footprint-space or UV-space rectangular bounds used by the projection pass.

For `ProjectionPassConfig.footprint_bounds`: coordinates are in millimetres.
For `ProjectionPassConfig.uv_bounds`: coordinates are normalized [0.0, 1.0].
[derive(Clone, Copy, Debug, PartialEq)]

## Methods

- `min_mm`
- `max_mm`

## Source
Lines 14–17 in `crates/oxide-renderer/src/pcb3d/projection.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [projection](/crates/oxide-renderer/src/pcb3d/projection.md) |
