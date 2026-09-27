---
okf_version: "0.2"
type: Function
title: translate_profile_with_pad
description: "Translate a sketch-profile pad's loop so it tracks the pad to"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/translate_profile_with_pad
language: rust
---

# translate_profile_with_pad

Translate a sketch-profile pad's loop so it tracks the pad to

## Signature

```rust
fn translate_profile_with_pad(
    sketch: &mut SketchData,
    centre: SketchEntityId,
    new_position: (f64, f64),
) -> HashSet<SketchEntityId>
```

## Docstring

Translate a sketch-profile pad's loop so it tracks the pad to
`new_position`, returning every Point it moved so the caller can
keep the delta off them a second time. No-op (and empty) for
non-profile pads, for a zero delta, or when the loop can't be
traced.

Raw-position mutation matches how a user-driven sketch drag behaves
(`SketchEdit::MovePoint` writes entity x/y and lets the next solve
run): a uniform translate preserves translation-invariant
constraints, and anything anchored absolutely re-asserts itself on
the next solve — the same outcome as dragging the loop by hand.

An untraceable loop (open / branching after a Sketch-mode edit)
leaves the profile put. That is not silent: the bake re-walks the
same loop and pushes its own "trace failed … falling back to Rect"
warning.

## Source
Lines 483–511 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.md) |
| calls | [profile_seed_line](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/profile_seed_line.md) |
| calls | [point_xy_of](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/point_xy_of.md) |
| calls | [trace_closed_profile_entities](/crates/oxide-bake/src/profile/trace_closed_profile_entities.md) |
| calls | [set_point_xy](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers/set_point_xy.md) |
| called_by | [mirror_move_pad_in_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_move_pad_in_sketch.md) |
