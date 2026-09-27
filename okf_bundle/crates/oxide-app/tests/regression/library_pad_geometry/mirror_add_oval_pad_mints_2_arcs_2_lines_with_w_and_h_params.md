---
okf_version: "0.2"
type: Function
title: mirror_add_oval_pad_mints_2_arcs_2_lines_with_w_and_h_params
description: v0.24 Track A5 — placing an Oval pad in Pads mode should mirror
resource: crates/oxide-app/tests/regression/library_pad_geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_pad_geometry/mirror_add_oval_pad_mints_2_arcs_2_lines_with_w_and_h_params
language: rust
---

# mirror_add_oval_pad_mints_2_arcs_2_lines_with_w_and_h_params

v0.24 Track A5 — placing an Oval pad in Pads mode should mirror

## Signature

```rust
fn mirror_add_oval_pad_mints_2_arcs_2_lines_with_w_and_h_params()
```

## Decorators

- `test`

## Docstring

v0.24 Track A5 — placing an Oval pad in Pads mode should mirror
into the sketch as the full Fusion-parity stadium primitive:
- 1 centre Point
- 4 bbox corner Points
- 4 arc-anchor Points (where the rounded ends meet the
straight edges)
- 2 Arc-centre Points (offset inward from the short-axis edges
by half the short axis)
= 11 Points
+ 2 long-axis Lines + 2 short-axis Arcs = 15 entities
`pad.shape_params` records `"width" -> width_<slug>` and
`"height" -> height_<slug>` so the Properties panel can surface
both as editable rows.
[test]

## Source
Lines 584–720 in `crates/oxide-app/tests/regression/library_pad_geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_pad_geometry](/crates/oxide-app/tests/regression/library_pad_geometry.md) |
| calls | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
