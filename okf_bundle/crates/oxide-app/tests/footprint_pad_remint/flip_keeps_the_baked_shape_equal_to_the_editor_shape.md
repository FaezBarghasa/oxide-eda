---
okf_version: "0.2"
type: Function
title: flip_keeps_the_baked_shape_equal_to_the_editor_shape
description: "THE INVARIANT (b). `oxide_bake::pad` reads `PadAttr::shape` off the"
resource: crates/oxide-app/tests/footprint_pad_remint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_remint/flip_keeps_the_baked_shape_equal_to_the_editor_shape
language: rust
---

# flip_keeps_the_baked_shape_equal_to_the_editor_shape

THE INVARIANT (b). `oxide_bake::pad` reads `PadAttr::shape` off the

## Signature

```rust
fn flip_keeps_the_baked_shape_equal_to_the_editor_shape()
```

## Decorators

- `test`

## Docstring

THE INVARIANT (b). `oxide_bake::pad` reads `PadAttr::shape` off the
sketch, so that field IS the baked shape. After a flip the editor
pad's chamfer corners are mirrored; if the sketch attribute still
carries the pre-flip corners then the editor and the bake disagree
about the pad and the exported footprint chamfers the wrong corner.

Asserting the editor value alone cannot see this — it is the two
representations AGREEING that matters.
[test]

## Source
Lines 222–283 in `crates/oxide-app/tests/footprint_pad_remint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_remint](/crates/oxide-app/tests/footprint_pad_remint.md) |
| calls | [editor_with_minted_pad](/crates/oxide-app/tests/footprint_pad_remint/editor_with_minted_pad.md) |
| calls | [chamfered_repro_pad](/crates/oxide-app/tests/footprint_pad_remint/chamfered_repro_pad.md) |
| calls | [dispatch](/crates/oxide-app/tests/footprint_pad_remint/dispatch.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
