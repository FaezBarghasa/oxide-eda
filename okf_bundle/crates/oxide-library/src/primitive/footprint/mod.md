---
okf_version: "0.2"
type: Module
title: footprint
description: "`Footprint` primitive — PCB-side reusable shape."
resource: crates/oxide-library/src/primitive/footprint/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/primitive/footprint/mod
language: rust
---

# footprint

`Footprint` primitive — PCB-side reusable shape.

## Docstring

`Footprint` primitive — PCB-side reusable shape.

Per `v0.9-refactor-2-plan.md` §2.2, a `Footprint` carries:
- typed pad list,
- courtyard polygon,
- silk / fab graphics for both copper sides,
- an embedded [`Body3D`] (drives Oxide's procedural 3D render),
- an optional [`StepAttachment`] (mech-CAD STEP file, content-hashed).

Two MPNs sharing a SOIC-8 footprint reference the same `Footprint` UUID
via `Component::footprint_ref` — the geometry lives once and accumulates
fixes over time.

## Relationships

| Type | Target |
|------|--------|
| related | [FpGraphicKind](/crates/oxide-library/src/primitive/footprint/mod/FpGraphicKind.md) |
| related | [FpGraphic](/crates/oxide-library/src/primitive/footprint/mod/FpGraphic.md) |
| related | [BodyShape](/crates/oxide-library/src/primitive/footprint/mod/BodyShape.md) |
| related | [Body3D](/crates/oxide-library/src/primitive/footprint/mod/Body3D.md) |
| related | [default](/crates/oxide-library/src/primitive/footprint/mod/default.md) |
| related | [default](/crates/oxide-library/src/primitive/footprint/mod/default.md) |
| related | [NetRef](/crates/oxide-library/src/primitive/footprint/mod/NetRef.md) |
| related | [named](/crates/oxide-library/src/primitive/footprint/mod/named.md) |
| related | [named](/crates/oxide-library/src/primitive/footprint/mod/named.md) |
| related | [PourFillType](/crates/oxide-library/src/primitive/footprint/mod/PourFillType.md) |
| related | [ThermalReliefStyle](/crates/oxide-library/src/primitive/footprint/mod/ThermalReliefStyle.md) |
| related | [FpPour](/crates/oxide-library/src/primitive/footprint/mod/FpPour.md) |
| related | [KeepoutForbid](/crates/oxide-library/src/primitive/footprint/mod/KeepoutForbid.md) |
| related | [FpKeepout](/crates/oxide-library/src/primitive/footprint/mod/FpKeepout.md) |
| related | [FpCutout](/crates/oxide-library/src/primitive/footprint/mod/FpCutout.md) |
| related | [default_true](/crates/oxide-library/src/primitive/footprint/mod/default_true.md) |
| related | [VScoreSide](/crates/oxide-library/src/primitive/footprint/mod/VScoreSide.md) |
| related | [FpVScore](/crates/oxide-library/src/primitive/footprint/mod/FpVScore.md) |
| related | [FpMaskOpening](/crates/oxide-library/src/primitive/footprint/mod/FpMaskOpening.md) |
| related | [FpPasteAperture](/crates/oxide-library/src/primitive/footprint/mod/FpPasteAperture.md) |
| related | [StepAttachment](/crates/oxide-library/src/primitive/footprint/mod/StepAttachment.md) |
| related | [Footprint](/crates/oxide-library/src/primitive/footprint/mod/Footprint.md) |
| related | [ComponentType](/crates/oxide-library/src/primitive/footprint/mod/ComponentType.md) |
| related | [label](/crates/oxide-library/src/primitive/footprint/mod/label.md) |
| related | [is_default](/crates/oxide-library/src/primitive/footprint/mod/is_default.md) |
| related | [label](/crates/oxide-library/src/primitive/footprint/mod/label.md) |
| related | [is_default](/crates/oxide-library/src/primitive/footprint/mod/is_default.md) |
| related | [fmt](/crates/oxide-library/src/primitive/footprint/mod/fmt.md) |
| related | [fmt](/crates/oxide-library/src/primitive/footprint/mod/fmt.md) |
| related | [default_footprint_version](/crates/oxide-library/src/primitive/footprint/mod/default_footprint_version.md) |
| related | [default_schema_v2](/crates/oxide-library/src/primitive/footprint/mod/default_schema_v2.md) |
| related | [FootprintFile](/crates/oxide-library/src/primitive/footprint/mod/FootprintFile.md) |
| related | [default_footprint_format](/crates/oxide-library/src/primitive/footprint/mod/default_footprint_format.md) |
| related | [FootprintFileWire](/crates/oxide-library/src/primitive/footprint/mod/FootprintFileWire.md) |
| related | [FootprintWire](/crates/oxide-library/src/primitive/footprint/mod/FootprintWire.md) |
| related | [from_footprint](/crates/oxide-library/src/primitive/footprint/mod/from_footprint.md) |
| related | [from_bytes](/crates/oxide-library/src/primitive/footprint/mod/from_bytes.md) |
| related | [from_toml_str](/crates/oxide-library/src/primitive/footprint/mod/from_toml_str.md) |
| related | [to_toml_string](/crates/oxide-library/src/primitive/footprint/mod/to_toml_string.md) |
| related | [get_footprint](/crates/oxide-library/src/primitive/footprint/mod/get_footprint.md) |
| related | [get_footprint_mut](/crates/oxide-library/src/primitive/footprint/mod/get_footprint_mut.md) |
| related | [from_footprint](/crates/oxide-library/src/primitive/footprint/mod/from_footprint.md) |
| related | [from_bytes](/crates/oxide-library/src/primitive/footprint/mod/from_bytes.md) |
| related | [from_toml_str](/crates/oxide-library/src/primitive/footprint/mod/from_toml_str.md) |
| related | [to_toml_string](/crates/oxide-library/src/primitive/footprint/mod/to_toml_string.md) |
| related | [get_footprint](/crates/oxide-library/src/primitive/footprint/mod/get_footprint.md) |
| related | [get_footprint_mut](/crates/oxide-library/src/primitive/footprint/mod/get_footprint_mut.md) |
| related | [empty](/crates/oxide-library/src/primitive/footprint/mod/empty.md) |
| related | [empty](/crates/oxide-library/src/primitive/footprint/mod/empty.md) |
| related | [chrono](/_dependencies/cargo/chrono.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
