---
okf_version: "0.2"
type: Function
title: mirror_add_chamfered_pad_mints_anchors_per_enabled_corner
description: v0.24 Track A6 — placing a Chamfered pad in Pads mode mirrors
resource: crates/oxide-app/tests/regression/library_pad_geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_pad_geometry/mirror_add_chamfered_pad_mints_anchors_per_enabled_corner
language: rust
---

# mirror_add_chamfered_pad_mints_anchors_per_enabled_corner

v0.24 Track A6 — placing a Chamfered pad in Pads mode mirrors

## Signature

```rust
fn mirror_add_chamfered_pad_mints_anchors_per_enabled_corner()
```

## Decorators

- `test`

## Docstring

v0.24 Track A6 — placing a Chamfered pad in Pads mode mirrors
into the sketch as a parametric outline. With only the top_left +
top_right corners enabled, the mint should:
- 1 centre Point + 4 bbox corner Points = 5 Points (baseline).
- Per ENABLED corner: 2 anchor Points (8 entries' worth ÷ 2
corners = 4 anchor Points total).
- A single shared `chamfer_len_<slug>` sketch parameter.
- Per-corner sidecar keys recording each anchor's UUID so a
future Unlink-chamfer-length action can resolve them.
[test]

## Source
Lines 838–963 in `crates/oxide-app/tests/regression/library_pad_geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_pad_geometry](/crates/oxide-app/tests/regression/library_pad_geometry.md) |
| calls | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
