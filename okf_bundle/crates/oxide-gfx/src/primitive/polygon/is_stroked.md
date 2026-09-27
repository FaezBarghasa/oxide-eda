---
okf_version: "0.2"
type: Function
title: is_stroked
description: "Whether this contour carries a stroked outline on top of its fill: it"
resource: crates/oxide-gfx/src/primitive/polygon.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-gfx/src/primitive/polygon/is_stroked
language: rust
---

# is_stroked

Whether this contour carries a stroked outline on top of its fill: it

## Signature

```rust
impl GpuPolygon { pub fn is_stroked(&self) -> bool }
```

## Visibility

- `pub`

## Docstring

Whether this contour carries a stroked outline on top of its fill: it
needs both a stroke colour and a positive width. Shared CPU↔GPU
predicate — the GPU tessellator (`append_stroke`) and the parity test
both use it.

NOTE: the CPU `renderer_scene_canvas`/`pcb_canvas` polygon paths stroke
whenever `stroke_color` is `Some`, even at zero width (clamped to a
minimum). That laxer rule is a known CPU↔GPU divergence the parity test
documents; this predicate is the GPU-side truth.

## Source
Lines 26–28 in `crates/oxide-gfx/src/primitive/polygon.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [polygon](/crates/oxide-gfx/src/primitive/polygon.md) |
