---
okf_version: "0.2"
type: Function
title: on_segment
resource: crates/oxide-router/src/geometry/collision.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:58:57Z"
concept_id: crates/oxide-router/src/geometry/collision/on_segment
language: rust
---

# on_segment

## Signature

```rust
fn on_segment(p: Point2D, q: Point2D, r: Point2D) -> bool
```

## Source
Lines 96–98 in `crates/oxide-router/src/geometry/collision.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [collision](/crates/oxide-router/src/geometry/collision.md) |
| called_by | [intersects_segment](/crates/oxide-router/src/geometry/collision/intersects_segment.md) |
