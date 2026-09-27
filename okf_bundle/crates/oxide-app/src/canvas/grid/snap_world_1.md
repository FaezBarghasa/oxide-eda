---
okf_version: "0.2"
type: Function
title: snap_world
description: Snap a world-space coordinate to the nearest grid point.
resource: crates/oxide-app/src/canvas/grid.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/canvas/grid/snap_world_1
language: rust
---

# snap_world

Snap a world-space coordinate to the nearest grid point.

## Signature

```rust
pub fn snap_world(&self, world: Point) -> Point
```

## Visibility

- `pub`

## Docstring

Snap a world-space coordinate to the nearest grid point.

## Source
Lines 59–65 in `crates/oxide-app/src/canvas/grid.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [grid](/crates/oxide-app/src/canvas/grid.md) |
