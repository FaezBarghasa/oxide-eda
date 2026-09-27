---
okf_version: "0.2"
type: Function
title: chamfered_pad_with_2_enabled_corners_has_2_chamfer_cuts
description: "Phase-5 #7 — Place a Chamfered pad with exactly two corners"
resource: crates/oxide-app/tests/regression/library_cross_track.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_cross_track/chamfered_pad_with_2_enabled_corners_has_2_chamfer_cuts
language: rust
---

# chamfered_pad_with_2_enabled_corners_has_2_chamfer_cuts

Phase-5 #7 — Place a Chamfered pad with exactly two corners

## Signature

```rust
fn chamfered_pad_with_2_enabled_corners_has_2_chamfer_cuts()
```

## Decorators

- `test`

## Docstring

Phase-5 #7 — Place a Chamfered pad with exactly two corners
enabled (top_left + top_right). The mint should produce exactly
2 chamfer-cut Lines (one per enabled corner) plus 4 outline edge
Lines = 6 total. Each disabled corner should NOT contribute a
chamfer-cut; the bbox corner Points stay as 90° angles in the
outline.

Cross-track: drives the FootprintAddPad dispatcher (Track A6
mint) end-to-end with a non-default ChamferedCorners flagset, so
the placement flow's path-keyed state lookup + Pads-mode-aware
mirror branch + per-corner sidecar bookkeeping all run together.
[test]

## Source
Lines 1280–1425 in `crates/oxide-app/tests/regression/library_cross_track.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_cross_track](/crates/oxide-app/tests/regression/library_cross_track.md) |
| calls | [fixture_empty_footprint_editor](/crates/oxide-app/tests/regression/library_cross_track/fixture_empty_footprint_editor.md) |
| calls | [set_pad_defaults](/crates/oxide-app/tests/regression/library_cross_track/set_pad_defaults.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
