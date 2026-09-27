---
okf_version: "0.2"
type: Function
title: pad_attr_from_editor_pad
description: "Build a sketch-side `PadAttr` from an `EditorPad`. Carries number /"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/pad_attr_from_editor_pad
language: rust
---

# pad_attr_from_editor_pad

Build a sketch-side `PadAttr` from an `EditorPad`. Carries number /

## Signature

```rust
pub(super) fn pad_attr_from_editor_pad(pad: &EditorPad) -> PadAttr
```

## Visibility

- `pub(super)`

## Docstring

Build a sketch-side `PadAttr` from an `EditorPad`. Carries number /
kind / side / shape + size expressions + drill spec. Other PadAttr
fields default; the v0.22 mirror path overwrites them.

## Source
Lines 136–166 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [attr](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr.md) |
| calls | [map_kind](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/map_kind.md) |
| calls | [map_side](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/map_side.md) |
| calls | [map_shape](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/map_shape.md) |
| calls | [rotation_expr](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/rotation_expr.md) |
| called_by | [auto_mint_for_literal_pads](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/auto_mint_for_literal_pads.md) |
| called_by | [mint_pad_entities](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mint_pad_entities.md) |
