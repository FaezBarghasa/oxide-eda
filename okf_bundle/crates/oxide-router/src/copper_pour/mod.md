---
okf_version: "0.2"
type: Module
title: copper_pour
description: "Smart Dynamic Copper Polygonal Pour & Teardrop Generation Engine."
resource: crates/oxide-router/src/copper_pour/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T04:53:15Z"
concept_id: crates/oxide-router/src/copper_pour/mod
language: rust
---

# copper_pour

Smart Dynamic Copper Polygonal Pour & Teardrop Generation Engine.

## Docstring

Smart Dynamic Copper Polygonal Pour & Teardrop Generation Engine.

Handles prioritized copper zone clipping, thermal relief spoke calculations (ortho/diagonal),
minimum area island removal, and automated curvilinear teardrop fillet generation on tracks/vias/pads.

## Relationships

| Type | Target |
|------|--------|
| related | [ThermalReliefStyle](/crates/oxide-router/src/copper_pour/mod/ThermalReliefStyle.md) |
| related | [CopperZoneConfig](/crates/oxide-router/src/copper_pour/mod/CopperZoneConfig.md) |
| related | [default](/crates/oxide-router/src/copper_pour/mod/default.md) |
| related | [default](/crates/oxide-router/src/copper_pour/mod/default.md) |
| related | [ThermalSpoke](/crates/oxide-router/src/copper_pour/mod/ThermalSpoke.md) |
| related | [CopperPourEngine](/crates/oxide-router/src/copper_pour/mod/CopperPourEngine.md) |
| related | [generate_thermal_spokes](/crates/oxide-router/src/copper_pour/mod/generate_thermal_spokes.md) |
| related | [is_valid_island](/crates/oxide-router/src/copper_pour/mod/is_valid_island.md) |
| related | [generate_thermal_spokes](/crates/oxide-router/src/copper_pour/mod/generate_thermal_spokes.md) |
| related | [is_valid_island](/crates/oxide-router/src/copper_pour/mod/is_valid_island.md) |
| related | [TeardropGenerator](/crates/oxide-router/src/copper_pour/mod/TeardropGenerator.md) |
| related | [generate_teardrop_for_pad](/crates/oxide-router/src/copper_pour/mod/generate_teardrop_for_pad.md) |
| related | [apply_teardrops_to_path](/crates/oxide-router/src/copper_pour/mod/apply_teardrops_to_path.md) |
| related | [generate_teardrop_for_pad](/crates/oxide-router/src/copper_pour/mod/generate_teardrop_for_pad.md) |
| related | [apply_teardrops_to_path](/crates/oxide-router/src/copper_pour/mod/apply_teardrops_to_path.md) |
| related | [test_thermal_relief_spokes](/crates/oxide-router/src/copper_pour/mod/test_thermal_relief_spokes.md) |
| related | [test_island_area_removal](/crates/oxide-router/src/copper_pour/mod/test_island_area_removal.md) |
| related | [test_teardrop_generation](/crates/oxide-router/src/copper_pour/mod/test_teardrop_generation.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
