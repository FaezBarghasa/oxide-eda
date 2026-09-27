---
okf_version: "0.2"
type: Function
title: sync_pads_to_primitive
description: Write the canvas-side pad list back onto the primitive. Called
resource: crates/oxide-app/src/library/editor/footprint/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/mod/sync_pads_to_primitive_1
language: rust
---

# sync_pads_to_primitive

Write the canvas-side pad list back onto the primitive. Called

## Signature

```rust
pub fn sync_pads_to_primitive(canvas: &Self, fp: &mut Footprint)
```

## Visibility

- `pub`

## Docstring

Write the canvas-side pad list back onto the primitive. Called
after every mutation so the saved row sees the current pad
layout. Other Footprint fields (graphics, body_3d, etc.) are
left untouched — they're edited by their own panes.

## Source
Lines 678–693 in `crates/oxide-app/src/library/editor/footprint/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/footprint/state/mod.md) |
| calls | [mirror_pad_attrs_into_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/mirror_pad_attrs_into_sketch.md) |
