---
okf_version: "0.2"
type: Function
title: sidecar
description: "Resolve a `shape_params` UUID-slug sidecar into its entity id."
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/sidecar
language: rust
---

# sidecar

Resolve a `shape_params` UUID-slug sidecar into its entity id.

## Signature

```rust
fn sidecar(pad: &EditorPad, key: &str) -> SketchEntityId
```

## Docstring

Resolve a `shape_params` UUID-slug sidecar into its entity id.

## Source
Lines 307–313 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.md) |
| calls | [SketchEntityId](/crates/oxide-sketch/src/id/SketchEntityId.md) |
| called_by | [mirror_move_oval_translates_anchor_sidecars](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_move_oval_translates_anchor_sidecars.md) |
| called_by | [mirror_move_roundrect_translates_anchors_and_arc_centres](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_move_roundrect_translates_anchors_and_arc_centres.md) |
| called_by | [roundrect_arc_centres](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/roundrect_arc_centres.md) |
