---
okf_version: "0.2"
type: Function
title: triangle_area
resource: crates/oxide-gfx/src/pipeline/polygon.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/polygon/triangle_area
language: rust
---

# triangle_area

## Signature

```rust
fn triangle_area(a: [f32; 2], b: [f32; 2], c: [f32; 2]) -> f64
```

## Source
Lines 401–403 in `crates/oxide-gfx/src/pipeline/polygon.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [polygon](/crates/oxide-gfx/src/pipeline/polygon.md) |
| calls | [shoelace_area](/crates/oxide-gfx/src/pipeline/polygon/shoelace_area.md) |
| called_by | [concave_contour_fills_exactly_its_own_area](/crates/oxide-gfx/src/pipeline/polygon/concave_contour_fills_exactly_its_own_area.md) |
