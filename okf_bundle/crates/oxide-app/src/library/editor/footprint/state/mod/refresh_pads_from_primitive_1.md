---
okf_version: "0.2"
type: Function
title: refresh_pads_from_primitive
description: "v0.22 Phase D2 — Inverse of `sync_pads_to_primitive`. After a"
resource: crates/oxide-app/src/library/editor/footprint/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/mod/refresh_pads_from_primitive_1
language: rust
---

# refresh_pads_from_primitive

v0.22 Phase D2 — Inverse of `sync_pads_to_primitive`. After a

## Signature

```rust
pub fn refresh_pads_from_primitive(&mut self, fp: &Footprint)
```

## Visibility

- `pub`

## Docstring

v0.22 Phase D2 — Inverse of `sync_pads_to_primitive`. After a
Sketch-mode solve+bake regenerates `Footprint::pads` from the
sketch source-of-truth, refresh the editor-side `pads` cache.

`sketch_entity_id`, `corner_entity_ids`, and `shape_params` (v0.24
Track A4) don't round-trip through `Pad`, so we re-attach them
by matching `pad.number`.

A pad matched in `old_links` keeps its full editor-side link. A
pad NOT in `old_links` — one that first appears from the sketch
side (e.g. "Make Pad from Profile", which mints a `PadAttr`-
carrying entity directly on the sketch and only reaches
`state.pads` after the next bake) — is relinked from the
authoritative source: the sketch entity whose `PadAttr.number`
matches. Without this fallback such a pad stays permanently
`sketch_entity_id: None`, so a Pads-mode move can't mirror into
the sketch and the pad snaps back to its original position on the
next bake.

The number match is only applied where the number identifies ONE
pad on each side. Pad numbers are not unique in oxide (a
shared-designator row / thermal / shield set is normal), and a
last-wins number map hands several pads the same
`sketch_entity_id` — after which a Pads-mode delete of one runs
the delete mirror over another pad's geometry and its copper
silently disappears from the bake. Ambiguous numbers are left
unlinked here and offered to `relink_pads_to_sketch`, which
disambiguates by exact position and refuses if even that ties.

## Source
Lines 658–672 in `crates/oxide-app/src/library/editor/footprint/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/footprint/state/mod.md) |
| calls | [carry_links_by_unique_number](/crates/oxide-app/src/library/editor/footprint/state/pad/carry_links_by_unique_number.md) |
| calls | [relink_pads_to_sketch](/crates/oxide-app/src/library/editor/footprint/state/pad/relink_pads_to_sketch.md) |
