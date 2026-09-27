---
okf_version: "0.2"
type: Function
title: build_ring
description: Build a closed ring from a slice of corner positions. The
resource: crates/oxide-sketch/src/geom/boolean_general.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/boolean_general/build_ring
language: rust
---

# build_ring

Build a closed ring from a slice of corner positions. The

## Signature

```rust
fn build_ring(positions: &[Point2]) -> Option<Vec<Vertex>>
```

## Docstring

Build a closed ring from a slice of corner positions. The
returned Vec's `next` / `prev` form a circular doubly-linked
list. Returns `None` for a degenerate input (< 3 vertices).

## Source
Lines 96–108 in `crates/oxide-sketch/src/geom/boolean_general.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [boolean_general](/crates/oxide-sketch/src/geom/boolean_general.md) |
| called_by | [polygon_op](/crates/oxide-sketch/src/geom/boolean_general/polygon_op.md) |
