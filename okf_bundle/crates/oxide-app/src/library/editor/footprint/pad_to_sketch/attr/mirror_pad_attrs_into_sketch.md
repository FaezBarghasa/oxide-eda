---
okf_version: "0.2"
type: Function
title: mirror_pad_attrs_into_sketch
description: v0.22 Phase D1 — Pads-mode → Sketch attribute mirror. Push every
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/mirror_pad_attrs_into_sketch
language: rust
---

# mirror_pad_attrs_into_sketch

v0.22 Phase D1 — Pads-mode → Sketch attribute mirror. Push every

## Signature

```rust
pub fn mirror_pad_attrs_into_sketch(pads: &[EditorPad], sketch: &mut SketchData)
```

## Visibility

- `pub`

## Docstring

v0.22 Phase D1 — Pads-mode → Sketch attribute mirror. Push every
editor-owned pad field onto the `PadAttr` carried by the pad's
centre `Point`, for each pad that has one.

Lives here rather than in `FootprintEditorState` because it is
pad↔sketch mapping policy, which is what this module is; the
editor-state module only calls it.

## Source
Lines 25–53 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [attr](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [mirror_rotation_expr](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/mirror_rotation_expr.md) |
| calls | [mirror_shape](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/mirror_shape.md) |
| called_by | [sync_pads_to_primitive](/crates/oxide-app/src/library/editor/footprint/state/mod/sync_pads_to_primitive.md) |
