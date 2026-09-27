---
okf_version: "0.2"
type: Module
title: unit
description: "Strict-unit parser and `Quantity` type for sketch expressions."
resource: crates/oxide-sketch/src/unit.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/unit
language: rust
---

# unit

Strict-unit parser and `Quantity` type for sketch expressions.

## Docstring

Strict-unit parser and `Quantity` type for sketch expressions.

Cleanroom implementation. No third-party constraint-solver source
consulted. Conversion factors are common physical constants:
- 1 mil = 0.0254 mm  (= 1 thou; SI-defined inch / 1000)
- 1 in  = 25.4   mm  (international inch, ISO 31-1)
- 1 µm  = 0.001  mm
- 1 deg = π/180  rad

See `docs/internal/SKETCH_MODE_v0.13_PLAN.md` Task 4.1.

## Relationships

| Type | Target |
|------|--------|
| related | [Unit](/crates/oxide-sketch/src/unit/Unit.md) |
| related | [family](/crates/oxide-sketch/src/unit/family.md) |
| related | [family](/crates/oxide-sketch/src/unit/family.md) |
| related | [UnitFamily](/crates/oxide-sketch/src/unit/UnitFamily.md) |
| related | [Quantity](/crates/oxide-sketch/src/unit/Quantity.md) |
| related | [length](/crates/oxide-sketch/src/unit/length.md) |
| related | [angle](/crates/oxide-sketch/src/unit/angle.md) |
| related | [count](/crates/oxide-sketch/src/unit/count.md) |
| related | [as_mm](/crates/oxide-sketch/src/unit/as_mm.md) |
| related | [as_rad](/crates/oxide-sketch/src/unit/as_rad.md) |
| related | [as_count](/crates/oxide-sketch/src/unit/as_count.md) |
| related | [length](/crates/oxide-sketch/src/unit/length.md) |
| related | [angle](/crates/oxide-sketch/src/unit/angle.md) |
| related | [count](/crates/oxide-sketch/src/unit/count.md) |
| related | [as_mm](/crates/oxide-sketch/src/unit/as_mm.md) |
| related | [as_rad](/crates/oxide-sketch/src/unit/as_rad.md) |
| related | [as_count](/crates/oxide-sketch/src/unit/as_count.md) |
| related | [UnitError](/crates/oxide-sketch/src/unit/UnitError.md) |
| related | [parse_quantity](/crates/oxide-sketch/src/unit/parse_quantity.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
| related | [thiserror](/_dependencies/cargo/thiserror.md) |
