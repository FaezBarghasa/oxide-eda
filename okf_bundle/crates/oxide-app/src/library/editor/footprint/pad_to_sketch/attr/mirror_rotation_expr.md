---
okf_version: "0.2"
type: Function
title: mirror_rotation_expr
description: "Rotation was the one geometry field the mirror never wrote, so a"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/mirror_rotation_expr
language: rust
---

# mirror_rotation_expr

Rotation was the one geometry field the mirror never wrote, so a

## Signature

```rust
pub fn mirror_rotation_expr(current: &mut Option<String>, deg: f64)
```

## Visibility

- `pub`

## Docstring

Rotation was the one geometry field the mirror never wrote, so a
sketch-baked pad came back at 0° while the literal `Pad` carried
the true angle. It is written now, but only over an expression this
module could itself have written.

The discriminator is "is this a bare numeric literal", NOT "does it
start with `=`". The `=` prefix is OPTIONAL throughout this
codebase — `oxide_sketch::solver::residual::resolve_dim` strips it
before parsing and `oxide_bake::pad::rotation_deg` does the same —
so a bare `leg_angle` or `apex_angle * 2` is a fully valid authored
parameter binding. Keying on `=` would destroy exactly those, with
no warning and no undo entry for the sketch attribute.

## Source
Lines 87–91 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [attr](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr.md) |
| calls | [rotation_expr](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/rotation_expr.md) |
| called_by | [mirror_pad_attrs_into_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/mirror_pad_attrs_into_sketch.md) |
