---
okf_version: "0.2"
type: Function
title: editing_corner_r_via_properties_updates_all_4_arcs
description: "Phase-5 #4 — Edit the shared `corner_r_<slug>` parameter via the"
resource: crates/oxide-app/tests/regression/library_cross_track.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_cross_track/editing_corner_r_via_properties_updates_all_4_arcs
language: rust
---

# editing_corner_r_via_properties_updates_all_4_arcs

Phase-5 #4 — Edit the shared `corner_r_<slug>` parameter via the

## Signature

```rust
fn editing_corner_r_via_properties_updates_all_4_arcs()
```

## Decorators

- `test`

## Docstring

Phase-5 #4 — Edit the shared `corner_r_<slug>` parameter via the
Properties-panel dispatch path. The parameter table rewrites
cleanly and solve resolves the new value for every consumer; the
`pad.stack.corner_radius_pct` mirror (Track A4) re-derives from the
resolved corner_r so the Pads-mode "Corner radius %" input stays
in sync with sketch-side edits.

NOTE: the v0.24 A2 mint stores arc-anchor / inset-corner Point
coordinates as literals; no constraint binds them to the shared
`corner_r` parameter, and there's no post-solve mirror analogous
to `mirror_solve_to_chamfer_anchors` for RoundRect arcs. So a
shared-param edit DOES propagate through resolved-parameters +
the pad-stack pct mirror, but the Arc geometry stays at the
literal mint-time radius until either (a) constraints bind the
arc-anchor Points to the shared parameter, or (b) a dedicated
`mirror_solve_to_round_rect_arcs` lands. **Deferred to Phase 6**
— flagged in the report. This test pins the surfaces that DO work
today: parameter rewrite + resolved-parameters propagation +
corner_radius_pct mirror.
[test]

## Source
Lines 620–731 in `crates/oxide-app/tests/regression/library_cross_track.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_cross_track](/crates/oxide-app/tests/regression/library_cross_track.md) |
| calls | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
