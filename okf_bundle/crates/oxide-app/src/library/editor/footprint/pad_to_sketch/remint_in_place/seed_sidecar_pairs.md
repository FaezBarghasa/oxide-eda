---
okf_version: "0.2"
type: Function
title: seed_sidecar_pairs
description: "Walk the pad's entities and the reference mint's in lockstep,"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place/seed_sidecar_pairs
language: rust
---

# seed_sidecar_pairs

Walk the pad's entities and the reference mint's in lockstep,

## Signature

```rust
fn seed_sidecar_pairs(
    pad: &EditorPad,
    reference: &EditorPad,
) -> Option<Vec<(SketchEntityId, SketchEntityId)>>
```

## Docstring

Walk the pad's entities and the reference mint's in lockstep,
pairing old id to new id. `None` when the two sets are not the same
shape — a differing key set, a corner array on one side only, or a
pair whose kinds disagree — which is what a shape swap looks like
from here.

A paired walk, not either of the single-sided sweeps: it descends
through Line / Arc / Circle because a RoundRect records only its
four Arcs on `shape_params` and the anchors and inset centres hang
off them.
The pairs the walk starts from: the centre, the bbox corners, and
whatever [`sidecar_id`] names on `shape_params`. `None` when the two
pads do not record the same THINGS — a differing key set, a corner
array on one side only, or a key that is an id on one side and a
parameter name on the other.

## Source
Lines 112–138 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [remint_in_place](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place.md) |
| calls | [sidecar_id](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/sidecar_id.md) |
| called_by | [pair_sidecar_entities](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place/pair_sidecar_entities.md) |
