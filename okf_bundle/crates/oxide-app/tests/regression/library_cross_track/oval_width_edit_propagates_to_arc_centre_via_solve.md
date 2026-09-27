---
okf_version: "0.2"
type: Function
title: oval_width_edit_propagates_to_arc_centre_via_solve
description: "Phase-5 #6 — Edit the `width_<slug>` parameter on an Oval pad."
resource: crates/oxide-app/tests/regression/library_cross_track.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_cross_track/oval_width_edit_propagates_to_arc_centre_via_solve
language: rust
---

# oval_width_edit_propagates_to_arc_centre_via_solve

Phase-5 #6 — Edit the `width_<slug>` parameter on an Oval pad.

## Signature

```rust
fn oval_width_edit_propagates_to_arc_centre_via_solve()
```

## Decorators

- `test`

## Docstring

Phase-5 #6 — Edit the `width_<slug>` parameter on an Oval pad.
The mint stores the long-axis literal in `width_<slug>` and minted
arc-anchor / arc-centre Points are placed at literal coordinates
derived from W and H. After a solve, the resolved-parameter map
must reflect the new W; the parameter table also rewrites cleanly.

NOTE: the v0.24 A5 mint records the new W in the parameter table
but does NOT yet reposition the literal Point coordinates of the
arc-anchor / arc-centre Points (no constraint binds them to W —
they're literal at mint time, just like a Fusion sketch where you
haven't drawn dimensions). Repositioning would require either
adding constraints linking the Point coords to `width` / `height`,
or a dedicated post-solve mirror analogous to
`mirror_solve_to_chamfer_anchors`. **Deferred to Phase 6** —
flagged in the report. This test pins the surface that DOES work:
the parameter table propagates the new W on resolve, so a
future constraint-bound Point would see the new value.
[test]

## Source
Lines 913–1023 in `crates/oxide-app/tests/regression/library_cross_track.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_cross_track](/crates/oxide-app/tests/regression/library_cross_track.md) |
| calls | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
