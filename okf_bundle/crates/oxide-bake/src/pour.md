---
okf_version: "0.2"
type: Module
title: pour
description: Pour bake — turns PourAttr-tagged closed sketch profiles into
resource: crates/oxide-bake/src/pour.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/pour
language: rust
---

# pour

Pour bake — turns PourAttr-tagged closed sketch profiles into

## Docstring

Pour bake — turns PourAttr-tagged closed sketch profiles into
`Footprint::pours: Vec<FpPour>` records.

Phase B / Stage 3 of the v0.14 sketch-mode plan. v0.14 records the
polygon boundary + all PourAttr metadata (layer, net, fill_type,
thermal_relief, clearance, min_thickness, priority). Actual fill
generation (polygon offset + raster fill + thermal-relief geometry)
lands in v0.15.

## Relationships

| Type | Target |
|------|--------|
| related | [bake_pours](/crates/oxide-bake/src/pour/bake_pours.md) |
| related | [build_ctx](/crates/oxide-bake/src/pour/build_ctx.md) |
| related | [opt_eval_mm](/crates/oxide-bake/src/pour/opt_eval_mm.md) |
| related | [map_fill](/crates/oxide-bake/src/pour/map_fill.md) |
| related | [map_thermal](/crates/oxide-bake/src/pour/map_thermal.md) |
| related | [solve](/crates/oxide-bake/src/pour/solve.md) |
| related | [rectangle_with_pour](/crates/oxide-bake/src/pour/rectangle_with_pour.md) |
| related | [bake_pour_records_metadata](/crates/oxide-bake/src/pour/bake_pour_records_metadata.md) |
| related | [bake_pour_thermal_disabled_maps_to_direct](/crates/oxide-bake/src/pour/bake_pour_thermal_disabled_maps_to_direct.md) |
| related | [bake_pour_outline_fill_maps_to_none](/crates/oxide-bake/src/pour/bake_pour_outline_fill_maps_to_none.md) |
