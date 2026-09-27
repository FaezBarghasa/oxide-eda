---
okf_version: "0.2"
type: Function
title: orientation
description: "0 -> collinear, 1 -> clockwise, 2 -> counterclockwise"
resource: crates/oxide-router/src/geometry/collision.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:58:57Z"
concept_id: crates/oxide-router/src/geometry/collision/orientation
language: rust
---

# orientation

0 -> collinear, 1 -> clockwise, 2 -> counterclockwise

## Signature

```rust
fn orientation(p: Point2D, q: Point2D, r: Point2D) -> i32
```

## Docstring

0 -> collinear, 1 -> clockwise, 2 -> counterclockwise

## Source
Lines 85–94 in `crates/oxide-router/src/geometry/collision.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [collision](/crates/oxide-router/src/geometry/collision.md) |
| called_by | [intersects_segment](/crates/oxide-router/src/geometry/collision/intersects_segment.md) |
