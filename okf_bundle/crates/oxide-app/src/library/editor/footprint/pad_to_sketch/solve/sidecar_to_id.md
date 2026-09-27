---
okf_version: "0.2"
type: Function
title: sidecar_to_id
description: "Resolve a `pad.shape_params[key]` UUID-slug sidecar into a"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve/sidecar_to_id
language: rust
---

# sidecar_to_id

Resolve a `pad.shape_params[key]` UUID-slug sidecar into a

## Signature

```rust
fn sidecar_to_id(pad: &super::super::state::EditorPad, key: &str) -> Option<SketchEntityId>
```

## Docstring

Resolve a `pad.shape_params[key]` UUID-slug sidecar into a
`SketchEntityId`. Returns `None` when the key is absent or its
value isn't a valid UUID.

## Source
Lines 337–340 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solve](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve.md) |
| called_by | [mirror_solve_to_round_rect_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve/mirror_solve_to_round_rect_geometry.md) |
| called_by | [move_anchor_via_sidecar](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve/move_anchor_via_sidecar.md) |
