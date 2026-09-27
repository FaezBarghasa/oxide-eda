---
okf_version: "0.2"
type: Module
title: pad
description: "Pad bake — turns SketchData + solved state into Vec<Pad>."
resource: crates/oxide-bake/src/pad.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-bake/src/pad
language: rust
---

# pad

Pad bake — turns SketchData + solved state into Vec<Pad>.

## Docstring

Pad bake — turns SketchData + solved state into Vec<Pad>.

Phase 7 Task 7.1 of the v0.13 sketch-mode plan. Walks every entity
tagged with [`PadAttr`], evaluates its expression strings against
the resolved parameter table, and emits one
[`oxide_library::primitive::footprint::Pad`] per entity.

Layer-name strings are produced by
[`oxide_types::layer::OxideLayer::altium_label`] so the baked
footprint matches the Altium-style label set ("Top Layer", "Top
Solder", "Top Paste", …) used by the rest of the Oxide PCB
taxonomy.

Cleanroom: no third-party constraint-solver, footprint-generator,
or numerical-library source consulted.

# Scope

v0.14 baseline (with v0.14 lib variant additions):
- `PadKind::{Smd, Tht, NptHole, ConnectorPad, Castellated, Fiducial}`
all bake to native `LibPadKind` variants. v0.13 used to fall
back Castellated→Tht and Fiducial→Smd with warnings; v0.14 ships
the variants directly so those warnings are gone.
- `PadShape::Chamfered { chamfer_ratio_expr, corners }` bakes to
`LibPadShape::Chamfered` natively (was RoundRect approximation
in v0.13).
- `PadShape::Custom(SketchProfile)` still falls back to
`LibPadShape::Rect` with a warning (sketch-profile bake lands
in v0.14.1).
- `PasteAperturePattern::{Grid, Custom}` warn + fall back to
`Single` (one aperture).
- Closed-profile attrs other than `pad` (silk / courtyard /
mask_opening / mask_exclude / paste_aperture / pour / keepout /
board_cutout / v_score) are baked by their respective
`crate::silk` / `crate::courtyard` / `crate::mask` / `crate::pour`
modules. `bake_pads` no longer warns about them — the dispatcher
invokes those modules separately.
- `keepout` / `board_cutout` / `v_score` bake lands in v0.14.1.
- `construction = true` entities are skipped silently.

## Relationships

| Type | Target |
|------|--------|
| related | [bake_pads](/crates/oxide-bake/src/pad/bake_pads.md) |
| related | [bake_one_pad](/crates/oxide-bake/src/pad/bake_one_pad.md) |
| related | [strip_eq_prefix](/crates/oxide-bake/src/pad/strip_eq_prefix.md) |
| related | [eval_mm](/crates/oxide-bake/src/pad/eval_mm.md) |
| related | [opt_eval_mm](/crates/oxide-bake/src/pad/opt_eval_mm.md) |
| related | [rotation_deg](/crates/oxide-bake/src/pad/rotation_deg.md) |
| related | [lib_kind](/crates/oxide-bake/src/pad/lib_kind.md) |
| related | [oxide_layer_id](/crates/oxide-bake/src/pad/oxide_layer_id.md) |
| related | [map_corners](/crates/oxide-bake/src/pad/map_corners.md) |
| related | [fiducial_layers](/crates/oxide-bake/src/pad/fiducial_layers.md) |
| related | [derive_layers](/crates/oxide-bake/src/pad/derive_layers.md) |
| related | [bake_shape](/crates/oxide-bake/src/pad/bake_shape.md) |
