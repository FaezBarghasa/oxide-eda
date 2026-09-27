---
okf_version: "0.2"
type: Module
title: units
description: Coordinate-unit helpers for the on-disk wire format.
resource: crates/oxide-types/src/format/units.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/format/units
language: rust
---

# units

Coordinate-unit helpers for the on-disk wire format.

## Docstring

Coordinate-unit helpers for the on-disk wire format.

On disk, schematic and PCB positions are emitted as integer
nanometres so the wire format is precision-stable across hand
edits. In memory, `Point` is `f64` mm. The conversion happens at
the row-row boundary: [`mm_to_nm`] / [`nm_to_mm`].

Pure code motion out of `mod.rs`; `pub(in crate::format)` so the
sibling row-translation modules can reach the converters, exactly
as when they lived in the single-file module.

NB: this module keeps its *own* `NM_PER_MM` (an `f64` scale used by
the file-format rounding logic) — the on-disk wire format is the
only place nanometres are used; `crate::coord` no longer defines a
competing integer-nm coordinate type (#394).

## Relationships

| Type | Target |
|------|--------|
| related | [mm_to_nm](/crates/oxide-types/src/format/units/mm_to_nm.md) |
| related | [nm_to_mm](/crates/oxide-types/src/format/units/nm_to_mm.md) |
