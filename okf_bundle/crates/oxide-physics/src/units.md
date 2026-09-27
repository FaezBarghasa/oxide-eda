---
okf_version: "0.2"
type: Module
title: units
description: Physical units and metric/imperial conversion helpers for Oxide EDA.
resource: crates/oxide-physics/src/units.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:09:12Z"
concept_id: crates/oxide-physics/src/units
language: rust
---

# units

Physical units and metric/imperial conversion helpers for Oxide EDA.

## Docstring

Physical units and metric/imperial conversion helpers for Oxide EDA.

Internal representation uses integer `Microns` (`i64`) to guarantee
exact arithmetic and eliminate floating-point drift.

## Relationships

| Type | Target |
|------|--------|
| related | [mm_to_microns](/crates/oxide-physics/src/units/mm_to_microns.md) |
| related | [microns_to_mm](/crates/oxide-physics/src/units/microns_to_mm.md) |
| related | [mils_to_microns](/crates/oxide-physics/src/units/mils_to_microns.md) |
| related | [microns_to_mils](/crates/oxide-physics/src/units/microns_to_mils.md) |
| related | [test_unit_conversions](/crates/oxide-physics/src/units/test_unit_conversions.md) |
