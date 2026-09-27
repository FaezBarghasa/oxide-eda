---
okf_version: "0.2"
type: Module
title: palette
description: The theme-derived colour set the Properties panel paints with.
resource: crates/oxide-app/src/panels/palette.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/palette
language: rust
---

# palette

The theme-derived colour set the Properties panel paints with.

## Docstring

The theme-derived colour set the Properties panel paints with.

Every Properties-panel renderer needs some subset of the same eight
colours, and before this module each one took them as loose
positional `Color` parameters. That had two costs. Deriving them was
copy-pasted into five call sites (`panels::properties::view_properties`,
`properties_parameters::general::view_properties_general`,
`element_properties::selected`, `element_properties::child_sheet`,
`footprint_editor_properties`), so a token change had to be applied
five times to stay consistent. And every colour has the same type, so
a transposed pair — `input_bg` where `input_bdr` was meant — compiled
silently and shipped the wrong chrome. Naming the fields makes that a
compile error instead.

## Relationships

| Type | Target |
|------|--------|
| related | [PanelPalette](/crates/oxide-app/src/panels/palette/PanelPalette.md) |
| related | [from_tokens](/crates/oxide-app/src/panels/palette/from_tokens.md) |
| related | [from_tokens](/crates/oxide-app/src/panels/palette/from_tokens.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
