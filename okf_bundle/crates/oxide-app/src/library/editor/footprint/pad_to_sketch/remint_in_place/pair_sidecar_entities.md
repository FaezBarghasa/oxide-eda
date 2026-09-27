---
okf_version: "0.2"
type: Function
title: pair_sidecar_entities
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place/pair_sidecar_entities
language: rust
---

# pair_sidecar_entities

## Signature

```rust
fn pair_sidecar_entities(
    pad: &EditorPad,
    reference: &EditorPad,
    reference_sketch: &SketchData,
    footprint: &Footprint,
) -> Option<Vec<(SketchEntityId, SketchEntityId)>>
```

## Source
Lines 140–190 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [remint_in_place](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place.md) |
| calls | [seed_sidecar_pairs](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place/seed_sidecar_pairs.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [remint_pad_geometry_in_place](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place/remint_pad_geometry_in_place.md) |
