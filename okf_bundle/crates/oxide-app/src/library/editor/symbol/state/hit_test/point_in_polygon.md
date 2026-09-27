---
okf_version: "0.2"
type: Function
title: point_in_polygon
description: Even-odd point-in-polygon test (implicitly-closed vertex ring) — a
resource: crates/oxide-app/src/library/editor/symbol/state/hit_test.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/hit_test/point_in_polygon
language: rust
---

# point_in_polygon

Even-odd point-in-polygon test (implicitly-closed vertex ring) — a

## Signature

```rust
fn point_in_polygon(p: [f64; 2], vertices: &[[f64; 2]]) -> bool
```

## Docstring

Even-odd point-in-polygon test (implicitly-closed vertex ring) — a
thin adapter over `oxide_sketch::geom::point_in_polygon`. Mirrors
the footprint canvas's own adapter of the same shared helper.

## Source
Lines 179–182 in `crates/oxide-app/src/library/editor/symbol/state/hit_test.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hit_test](/crates/oxide-app/src/library/editor/symbol/state/hit_test.md) |
| called_by | [hit_test_graphic_body](/crates/oxide-app/src/library/editor/symbol/state/hit_test/hit_test_graphic_body.md) |
