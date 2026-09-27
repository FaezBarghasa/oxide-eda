---
okf_version: "0.2"
type: Module
title: primitive
description: "Reusable geometry primitives — `Symbol`, `Footprint`, `SimModel`."
resource: crates/oxide-library/src/primitive/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/mod
language: rust
---

# primitive

Reusable geometry primitives — `Symbol`, `Footprint`, `SimModel`.

## Docstring

Reusable geometry primitives — `Symbol`, `Footprint`, `SimModel`.

Per `v0.9-refactor-2-plan.md` §2, primitives are addressed by
`(library_id, uuid)` tuples (a [`PrimitiveRef`]) and bound onto `Component`
revisions. Two MPNs sharing a SOIC-8 footprint reference the same
`Footprint` primitive — they don't carry their own copy.

Module layout matches the plan:
- [`Symbol`] / [`SymbolPin`] / [`PinDirection`] / [`PinOrientation`]
in [`symbol`]
- [`Footprint`] / [`Pad`] / [`PadKind`] / [`PadShape`] / [`Body3D`] /
[`BodyShape`] / [`StepAttachment`] / [`Drill`] / [`Polygon`] /
[`FpGraphic`] in [`footprint`]
- [`SimModel`] / [`SimKind`] in [`sim`]
- [`PrimitiveRef`] in [`ref_`]

## Relationships

| Type | Target |
|------|--------|
| related | [PrimitiveKind](/crates/oxide-library/src/primitive/mod/PrimitiveKind.md) |
