---
okf_version: "0.2"
type: Module
title: layers
description: Footprint editor layer registry — the seven Altium-spec layers a
resource: crates/oxide-app/src/library/editor/footprint/layers.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/layers
language: rust
---

# layers

Footprint editor layer registry — the seven Altium-spec layers a

## Docstring

Footprint editor layer registry — the seven Altium-spec layers a
footprint surfaces: F.Cu / B.Cu / F.SilkS / B.SilkS / F.Fab / B.Fab
/ Edge.Cuts (the courtyard polygon stays on Edge.Cuts in this Phase
2 cut; F.CrtYd / B.CrtYd land alongside the proper paste/mask
toggles in v0.9.x).

## Relationships

| Type | Target |
|------|--------|
| related | [FpLayer](/crates/oxide-app/src/library/editor/footprint/layers/FpLayer.md) |
| related | [label](/crates/oxide-app/src/library/editor/footprint/layers/label.md) |
| related | [standard_name](/crates/oxide-app/src/library/editor/footprint/layers/standard_name.md) |
| related | [from_standard_name](/crates/oxide-app/src/library/editor/footprint/layers/from_standard_name.md) |
| related | [color](/crates/oxide-app/src/library/editor/footprint/layers/color.md) |
| related | [label](/crates/oxide-app/src/library/editor/footprint/layers/label.md) |
| related | [standard_name](/crates/oxide-app/src/library/editor/footprint/layers/standard_name.md) |
| related | [from_standard_name](/crates/oxide-app/src/library/editor/footprint/layers/from_standard_name.md) |
| related | [color](/crates/oxide-app/src/library/editor/footprint/layers/color.md) |
| related | [LayerVisibility](/crates/oxide-app/src/library/editor/footprint/layers/LayerVisibility.md) |
| related | [default](/crates/oxide-app/src/library/editor/footprint/layers/default.md) |
| related | [default](/crates/oxide-app/src/library/editor/footprint/layers/default.md) |
| related | [get](/crates/oxide-app/src/library/editor/footprint/layers/get.md) |
| related | [toggle](/crates/oxide-app/src/library/editor/footprint/layers/toggle.md) |
| related | [get](/crates/oxide-app/src/library/editor/footprint/layers/get.md) |
| related | [toggle](/crates/oxide-app/src/library/editor/footprint/layers/toggle.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
