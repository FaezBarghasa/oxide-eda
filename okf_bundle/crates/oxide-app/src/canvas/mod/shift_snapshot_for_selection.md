---
okf_version: "0.2"
type: Function
title: shift_snapshot_for_selection
description: Map a relative x position within the Active Bar to a dropdown menu.
resource: crates/oxide-app/src/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/canvas/mod/shift_snapshot_for_selection
language: rust
---

# shift_snapshot_for_selection

Map a relative x position within the Active Bar to a dropdown menu.

## Signature

```rust
fn shift_snapshot_for_selection(
    snap: &crate::schematic_runtime::SchematicRenderSnapshot,
    selection: &[oxide_types::schematic::SelectedItem],
    dx: f64,
    dy: f64,
) -> crate::schematic_runtime::SchematicRenderSnapshot
```

## Docstring

Map a relative x position within the Active Bar to a dropdown menu.
Returns None for buttons without dropdowns (Select, Add Component).
Clone the snapshot and translate the world position of every item that
appears in `selection` by `(dx, dy)`. Used during drag-to-move so the live
render shows selected objects at the cursor position while the originals
are not drawn at their pre-drag location.

## Source
Lines 503–633 in `crates/oxide-app/src/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/canvas/mod.md) |
| called_by | [draw](/crates/oxide-app/src/canvas/mod/draw.md) |
