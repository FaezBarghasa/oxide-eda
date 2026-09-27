---
okf_version: "0.2"
type: Function
title: trace_closed_profile
description: "Trace a closed boundary starting at `start` and emit its boundary"
resource: crates/oxide-bake/src/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/profile/trace_closed_profile
language: rust
---

# trace_closed_profile

Trace a closed boundary starting at `start` and emit its boundary

## Signature

```rust
pub fn trace_closed_profile(
    sketch: &SketchData,
    solve: &FullSolveOutput,
    start: SketchEntityId,
) -> TraceResult
```

## Visibility

- `pub`

## Docstring

Trace a closed boundary starting at `start` and emit its boundary
polygon — [`trace_closed_profile_entities`] plus solved positions,
with Arc segments tessellated into [`ARC_SAMPLES`] interior
vertices.

## Source
Lines 220–253 in `crates/oxide-bake/src/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-bake/src/profile.md) |
| calls | [trace_closed_profile_entities](/crates/oxide-bake/src/profile/trace_closed_profile_entities.md) |
| calls | [push_arc_interior_if_arc](/crates/oxide-bake/src/profile/push_arc_interior_if_arc.md) |
| calls | [pos](/crates/oxide-bake/src/silk/pos.md) |
| called_by | [make_pad_from_profile](/crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge/make_pad_from_profile.md) |
| called_by | [bake_body3d](/crates/oxide-bake/src/body3d/bake_body3d.md) |
| called_by | [bake_courtyard](/crates/oxide-bake/src/courtyard/bake_courtyard.md) |
| called_by | [bake_cutouts](/crates/oxide-bake/src/cutout/bake_cutouts.md) |
| called_by | [bake_keepouts](/crates/oxide-bake/src/keepout/bake_keepouts.md) |
| called_by | [bake_layered](/crates/oxide-bake/src/mask/bake_layered.md) |
| called_by | [bake_shape](/crates/oxide-bake/src/pad/bake_shape.md) |
| called_by | [bake_pours](/crates/oxide-bake/src/pour/bake_pours.md) |
| called_by | [trace_arc_seed_walks_back_through_line](/crates/oxide-bake/src/profile/trace_arc_seed_walks_back_through_line.md) |
| called_by | [trace_construction_lines_skipped](/crates/oxide-bake/src/profile/trace_construction_lines_skipped.md) |
| called_by | [trace_d_shape_cw_arc_closes_lower_half](/crates/oxide-bake/src/profile/trace_d_shape_cw_arc_closes_lower_half.md) |
| called_by | [trace_d_shape_line_plus_arc_closes](/crates/oxide-bake/src/profile/trace_d_shape_line_plus_arc_closes.md) |
| called_by | [trace_rectangle_closes](/crates/oxide-bake/src/profile/trace_rectangle_closes.md) |
