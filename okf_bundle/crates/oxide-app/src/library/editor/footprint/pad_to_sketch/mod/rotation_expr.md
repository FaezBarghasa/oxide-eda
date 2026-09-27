---
okf_version: "0.2"
type: Function
title: rotation_expr
description: "The sketch-side expression for a pad's rotation. Shared by the"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/rotation_expr
language: rust
---

# rotation_expr

The sketch-side expression for a pad's rotation. Shared by the

## Signature

```rust
pub fn rotation_expr(deg: f64) -> String
```

## Visibility

- `pub`

## Docstring

The sketch-side expression for a pad's rotation. Shared by the
mint path (`pad_attr_from_editor_pad`) and the Pads→Sketch
attribute mirror in `sync_pads_to_primitive`, so both write the
identical string and the two persistence paths cannot drift.
Emits an explicit `deg` unit; `oxide_bake::pad` reads it back
through the Angle unit family.

## Source
Lines 50–52 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.md) |
| called_by | [mirror_rotation_expr](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/mirror_rotation_expr.md) |
| called_by | [pad_attr_from_editor_pad](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/pad_attr_from_editor_pad.md) |
