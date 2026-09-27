---
okf_version: "0.2"
type: Function
title: roundrect_arc_centres
description: "The `center` Point of each of RoundRect's 4 corner Arcs, resolved"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/roundrect_arc_centres
language: rust
---

# roundrect_arc_centres

The `center` Point of each of RoundRect's 4 corner Arcs, resolved

## Signature

```rust
fn roundrect_arc_centres(pad: &EditorPad, fp: &Footprint) -> Vec<(f64, f64)>
```

## Docstring

The `center` Point of each of RoundRect's 4 corner Arcs, resolved
through the same `corner_r_{c}_arc` sidecar keys
`mirror_move_roundrect_translates_anchors_and_arc_centres` uses —
independent of `pair_sidecar_entities`'s own internal traversal, so
this cannot pass merely because that traversal and this assertion
share a bug.

## Source
Lines 1027–1044 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.md) |
| calls | [sidecar](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/sidecar.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [point_of](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/point_of.md) |
| called_by | [assert_in_place_remint_matches_fresh_mint](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/assert_in_place_remint_matches_fresh_mint.md) |
