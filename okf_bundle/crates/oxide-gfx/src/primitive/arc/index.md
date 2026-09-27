# arc

## Classs

- [Arc](Arc.md) — Circular arc with start/end angles in radians.

## Functions

- [arc_is_full_turn_rad](arc_is_full_turn_rad.md) — `true` when an arc's raw (unwrapped) `start..end` span is a nonzero
- [ccw_wrapped_sweep_rad](ccw_wrapped_sweep_rad.md) — The single Rust-side authority for this codebase's arc-sweep
- [degenerate_point_and_ordinary_arc_are_not_full_turns](degenerate_point_and_ordinary_arc_are_not_full_turns.md) — `start == end` is a genuine zero-sweep point-arc (raw span ~0),
- [equal_start_and_end_is_zero_sweep_not_a_full_circle](equal_start_and_end_is_zero_sweep_not_a_full_circle.md) — A degenerate zero-sweep arc (start == end) stays a point, not
- [full_turn_pair_is_detected](full_turn_pair_is_detected.md) — A `0 -> 360°` (`0 -> TAU`) pair sweeps zero CCW but spans a full
- [non_wrapped_arc_sweeps_the_raw_difference](non_wrapped_arc_sweeps_the_raw_difference.md) — A non-wrapped arc (`start <= end`, no seam crossing) sweeps
- [pins_330_to_30_as_60_degrees_through_zero](pins_330_to_30_as_60_degrees_through_zero.md) — The bug this whole normalization pass exists to fix, pinned as
- [swapping_endpoints_complements_the_sweep](swapping_endpoints_complements_the_sweep.md) — Sign/direction sanity: swapping start and end complements the
