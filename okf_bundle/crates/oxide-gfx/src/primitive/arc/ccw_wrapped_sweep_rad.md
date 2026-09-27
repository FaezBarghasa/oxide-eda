---
okf_version: "0.2"
type: Function
title: ccw_wrapped_sweep_rad
description: "The single Rust-side authority for this codebase's arc-sweep"
resource: crates/oxide-gfx/src/primitive/arc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-gfx/src/primitive/arc/ccw_wrapped_sweep_rad
language: rust
---

# ccw_wrapped_sweep_rad

The single Rust-side authority for this codebase's arc-sweep

## Signature

```rust
pub fn ccw_wrapped_sweep_rad(start_angle: f32, end_angle: f32) -> f32
```

## Visibility

- `pub`

## Docstring

The single Rust-side authority for this codebase's arc-sweep
convention: `start_angle..end_angle` sweeps counter-clockwise
(increasing angle) from `start_angle`, wrapping through a full
turn when `end_angle < start_angle` — never the signed, unwrapped
`end - start` difference. Returns the sweep in `[0, TAU)` radians:
the angular distance travelled going CCW from `start_angle` to
reach `end_angle`.

This is the exact Rust equivalent of `normalize_angle(end_angle -
start_angle)` in `crates/oxide-gfx/src/shader/arc.wgsl`'s
`sdf_arc` (the GPU arc renderer) — same formula, same convention.
`crates/oxide-app/src/library/editor/symbol/state/hit_test.rs`'s
`Arc` hit-test arm and `rotation.rs`'s Arc rotate arm independently
implement the same wraparound rule against `SymbolGraphicKind::
Arc`'s degree-valued `start_deg`/`end_deg` (via `rem_euclid(360.0)`
combined with an `if s <= e { .. } else { .. }` branch) rather than
calling this function directly, since they operate in degrees on a
different (oxide-library) type — but the rule they implement is
this one. Any Rust code that needs the CCW-wraparound sweep of a
radian-valued arc (in particular the CPU canvas draw path, which
used to hand iced's arc builder a raw unnormalized `end - start`
and silently draw the wrong complement for any wrapped arc) must
call this function rather than re-deriving the formula.

## Source
Lines 43–46 in `crates/oxide-gfx/src/primitive/arc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [arc](/crates/oxide-gfx/src/primitive/arc.md) |
| called_by | [draw_arc_preview](/crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays/draw_arc_preview.md) |
| called_by | [arc_endpoint_handle_drag_survives_save_reload](/crates/oxide-app/src/library/editor/symbol/state/tests/arc_endpoint_handle_drag_survives_save_reload.md) |
| called_by | [rotated_wraparound_arc_hit_test_and_draw_sweep_agree](/crates/oxide-app/src/library/editor/symbol/state/tests/rotated_wraparound_arc_hit_test_and_draw_sweep_agree.md) |
| called_by | [arc_screen_span](/crates/oxide-app/src/renderer_scene_canvas/arc_screen_span.md) |
| called_by | [arc_is_full_turn_rad](/crates/oxide-gfx/src/primitive/arc/arc_is_full_turn_rad.md) |
| called_by | [equal_start_and_end_is_zero_sweep_not_a_full_circle](/crates/oxide-gfx/src/primitive/arc/equal_start_and_end_is_zero_sweep_not_a_full_circle.md) |
| called_by | [non_wrapped_arc_sweeps_the_raw_difference](/crates/oxide-gfx/src/primitive/arc/non_wrapped_arc_sweeps_the_raw_difference.md) |
| called_by | [pins_330_to_30_as_60_degrees_through_zero](/crates/oxide-gfx/src/primitive/arc/pins_330_to_30_as_60_degrees_through_zero.md) |
| called_by | [swapping_endpoints_complements_the_sweep](/crates/oxide-gfx/src/primitive/arc/swapping_endpoints_complements_the_sweep.md) |
