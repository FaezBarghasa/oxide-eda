---
okf_version: "0.2"
type: Function
title: bake_shape
description: "Map sketch `PadShape` to the library's baked `LibPadShape`."
resource: crates/oxide-bake/src/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-bake/src/pad/bake_shape
language: rust
---

# bake_shape

Map sketch `PadShape` to the library's baked `LibPadShape`.

## Signature

```rust
fn bake_shape(
    s: &PadShape,
    ctx: &EvalContext,
    warnings: &mut Vec<String>,
    pad_number: &str,
    sketch: &SketchData,
    solve: &FullSolveOutput,
    pad_position: [f64; 2],
) -> Result<LibPadShape, SketchError>
```

## Docstring

Map sketch `PadShape` to the library's baked `LibPadShape`.

## Source
Lines 413–495 in `crates/oxide-bake/src/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-bake/src/pad.md) |
| calls | [strip_eq_prefix](/crates/oxide-bake/src/pad/strip_eq_prefix.md) |
| calls | [map_corners](/crates/oxide-bake/src/pad/map_corners.md) |
| calls | [trace_closed_profile](/crates/oxide-bake/src/profile/trace_closed_profile.md) |
| called_by | [bake_one_pad](/crates/oxide-bake/src/pad/bake_one_pad.md) |
