---
okf_version: "0.2"
type: Module
title: labels
description: Label + field text placement helpers.
resource: crates/oxide-output/src/svg/labels.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/src/svg/labels
language: rust
---

# labels

Label + field text placement helpers.

## Docstring

Label + field text placement helpers.

Label font sizing, per-label colour, the spin/justify geometry that
positions net / global / hierarchical labels, and the `HAlign`/
`VAlign` → SVG alignment converters shared with note and field text.

Extracted verbatim from the SVG exporter (`svg/mod.rs`); pure code
motion, zero behaviour change.

## Relationships

| Type | Target |
|------|--------|
| related | [label_size_pt](/crates/oxide-output/src/svg/labels/label_size_pt.md) |
| related | [label_colour](/crates/oxide-output/src/svg/labels/label_colour.md) |
| related | [SpinStyle](/crates/oxide-output/src/svg/labels/SpinStyle.md) |
| related | [label_spin_style](/crates/oxide-output/src/svg/labels/label_spin_style.md) |
| related | [schematic_text_offset_net](/crates/oxide-output/src/svg/labels/schematic_text_offset_net.md) |
| related | [schematic_text_offset_hier](/crates/oxide-output/src/svg/labels/schematic_text_offset_hier.md) |
| related | [schematic_text_offset_global](/crates/oxide-output/src/svg/labels/schematic_text_offset_global.md) |
| related | [spin_text_style](/crates/oxide-output/src/svg/labels/spin_text_style.md) |
| related | [normalize_rotation](/crates/oxide-output/src/svg/labels/normalize_rotation.md) |
| related | [halign_to_svg](/crates/oxide-output/src/svg/labels/halign_to_svg.md) |
| related | [valign_to_svg](/crates/oxide-output/src/svg/labels/valign_to_svg.md) |
