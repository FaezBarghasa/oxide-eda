---
okf_version: "0.2"
type: Function
title: set_pad_defaults
description: "Phase-5 helper — set `state.next_pad_defaults.shape` so the next"
resource: crates/oxide-app/tests/regression/library_cross_track.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_cross_track/set_pad_defaults
language: rust
---

# set_pad_defaults

Phase-5 helper — set `state.next_pad_defaults.shape` so the next

## Signature

```rust
fn set_pad_defaults(
    app: &mut Oxide,
    path: &std::path::Path,
    shape: oxide_library::PadShape,
    size_mm: (f64, f64),
)
```

## Docstring

Phase-5 helper — set `state.next_pad_defaults.shape` so the next
`FootprintAddPad` dispatch mints a pad of the requested shape +
size. Mirrors what the Properties panel does when the user picks a
shape from the Pad Stack picker before clicking the canvas.

## Source
Lines 113–127 in `crates/oxide-app/tests/regression/library_cross_track.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_cross_track](/crates/oxide-app/tests/regression/library_cross_track.md) |
| called_by | [chamfered_pad_with_2_enabled_corners_has_2_chamfer_cuts](/crates/oxide-app/tests/regression/library_cross_track/chamfered_pad_with_2_enabled_corners_has_2_chamfer_cuts.md) |
| called_by | [place_round_rect_then_undo_restores_pre_place_state](/crates/oxide-app/tests/regression/library_cross_track/place_round_rect_then_undo_restores_pre_place_state.md) |
