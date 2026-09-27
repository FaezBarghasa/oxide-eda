---
okf_version: "0.2"
type: Function
title: snap_to_grid
description: "Snap each coordinate to the nearest multiple of `step`. Kills"
resource: crates/oxide-sketch/src/geom/simplify.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/simplify/snap_to_grid
language: rust
---

# snap_to_grid

Snap each coordinate to the nearest multiple of `step`. Kills

## Signature

```rust
pub fn snap_to_grid(polygon: &[Point2], step: f64) -> Vec<Point2>
```

## Visibility

- `pub`

## Docstring

Snap each coordinate to the nearest multiple of `step`. Kills
near-duplicate vertices that would otherwise fail dedup at a
looser eps. Use a step matching the user-facing precision —
too loose snaps two distinct features together; too tight is
a no-op.

## Source
Lines 90–98 in `crates/oxide-sketch/src/geom/simplify.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [simplify](/crates/oxide-sketch/src/geom/simplify.md) |
| called_by | [simplify_polygon](/crates/oxide-sketch/src/geom/simplify/simplify_polygon.md) |
| called_by | [snap_to_grid_rounds_to_step](/crates/oxide-sketch/src/geom/simplify/snap_to_grid_rounds_to_step.md) |
