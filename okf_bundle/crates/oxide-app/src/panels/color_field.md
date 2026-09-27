---
okf_version: "0.2"
type: Module
title: color_field
description: Reusable colour-selection field — one swatch button that expands
resource: crates/oxide-app/src/panels/color_field.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/color_field
language: rust
---

# color_field

Reusable colour-selection field — one swatch button that expands

## Docstring

Reusable colour-selection field — one swatch button that expands
into an inline preset palette (6×2 grid) plus a `Custom…` button
that opens the `iced_aw` HSV / RGB `ColorPicker` overlay.

Ported verbatim from the former inline `child_sheet_color_row` so
every call site (child-sheet border/fill, symbol graphic fill,
symbol local colours) renders an identical control. The widget is
message-generic: callers hand in the five messages the control can
emit (`on_toggle` / `on_advanced` / `on_cancel` / `on_pick` /
`on_clear`) and read the open-state back out of their own context.

Colours cross the boundary as `[u8; 4]` RGBA; the widget converts
to / from `iced::Color` internally (preset cells are opaque, the
HSV submit is quantised to 8-bit).

## Relationships

| Type | Target |
|------|--------|
| related | [ColorFieldProps](/crates/oxide-app/src/panels/color_field/ColorFieldProps.md) |
| related | [color_field](/crates/oxide-app/src/panels/color_field/color_field.md) |
| related | [color_to_rgba](/crates/oxide-app/src/panels/color_field/color_to_rgba.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
