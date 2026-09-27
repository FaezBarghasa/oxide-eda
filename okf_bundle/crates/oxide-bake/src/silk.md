---
okf_version: "0.2"
type: Module
title: silk
description: Silkscreen bake — turns SilkAttr-tagged sketch entities into
resource: crates/oxide-bake/src/silk.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-bake/src/silk
language: rust
---

# silk

Silkscreen bake — turns SilkAttr-tagged sketch entities into

## Docstring

Silkscreen bake — turns SilkAttr-tagged sketch entities into
`FpGraphic` Line / Arc / Circle entries on the matching layer
(`F.SilkS` / `B.SilkS` typically).

Phase B / Stage 3 of the v0.14 sketch-mode plan. Unlike courtyard /
mask / pour bakes, silk does NOT require closed-profile tracing —
each Line / Arc / Circle entity tagged with [`SilkAttr`] emits one
`FpGraphic` directly. Open paths are valid silkscreen geometry
(e.g. component outline reference marks).

v0.14 scope:
- `EntityKind::Line` → `FpGraphicKind::Line { from, to }`
- `EntityKind::Arc`  → `FpGraphicKind::Arc { center, radius,
start_deg, end_deg }`
- `EntityKind::Circle` → `FpGraphicKind::Circle { center, radius }`
- `EntityKind::Point` carrying a SilkAttr emits no graphic and
triggers a warning (a Point is not a renderable silk primitive).
- Construction entities are skipped silently.

Layer routing: SilkAttr.layer is used directly. `TopSilk` →
`silk_f`, `BottomSilk` → `silk_b`. Other layers (e.g. someone tags
a SilkAttr with TopAssembly) emit a warning and skip — silk
semantics only make sense on silk layers.

Cleanroom: per-entity translation only; no third-party
footprint-generator source consulted.

## Relationships

| Type | Target |
|------|--------|
| related | [bake_silk](/crates/oxide-bake/src/silk/bake_silk.md) |
| related | [entity_to_graphic](/crates/oxide-bake/src/silk/entity_to_graphic.md) |
| related | [pos](/crates/oxide-bake/src/silk/pos.md) |
| related | [solve](/crates/oxide-bake/src/silk/solve.md) |
| related | [empty_sketch](/crates/oxide-bake/src/silk/empty_sketch.md) |
| related | [bake_silk_line_to_top_silk](/crates/oxide-bake/src/silk/bake_silk_line_to_top_silk.md) |
| related | [bake_silk_circle_to_bottom_silk](/crates/oxide-bake/src/silk/bake_silk_circle_to_bottom_silk.md) |
| related | [bake_silk_construction_skipped](/crates/oxide-bake/src/silk/bake_silk_construction_skipped.md) |
| related | [bake_silk_wrong_layer_warns](/crates/oxide-bake/src/silk/bake_silk_wrong_layer_warns.md) |
