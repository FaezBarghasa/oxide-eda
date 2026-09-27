---
okf_version: "0.2"
type: Function
title: intersects_segment
description: Check if two line segments intersect (ignoring width).
resource: crates/oxide-router/src/geometry/collision.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:58:57Z"
concept_id: crates/oxide-router/src/geometry/collision/intersects_segment
language: rust
---

# intersects_segment

Check if two line segments intersect (ignoring width).

## Signature

```rust
impl LineSegment { pub fn intersects_segment(&self, other: &LineSegment) -> bool }
```

## Visibility

- `pub`

## Docstring

Check if two line segments intersect (ignoring width).

## Source
Lines 30–58 in `crates/oxide-router/src/geometry/collision.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [collision](/crates/oxide-router/src/geometry/collision.md) |
| calls | [orientation](/crates/oxide-router/src/geometry/collision/orientation.md) |
| calls | [on_segment](/crates/oxide-router/src/geometry/collision/on_segment.md) |
