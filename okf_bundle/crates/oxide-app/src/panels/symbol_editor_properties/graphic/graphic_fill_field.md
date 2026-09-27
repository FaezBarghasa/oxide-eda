---
okf_version: "0.2"
type: Function
title: graphic_fill_field
description: Fill colour row for a closed graphic (Rectangle / Circle) — the
resource: crates/oxide-app/src/panels/symbol_editor_properties/graphic.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/symbol_editor_properties/graphic/graphic_fill_field
language: rust
---

# graphic_fill_field

Fill colour row for a closed graphic (Rectangle / Circle) — the

## Signature

```rust
fn graphic_fill_field(
    idx: usize,
    fill: Option<[u8; 4]>,
    muted: Color,
    border_c: Color,
    picker: Option<crate::app::GraphicFillPicker>,
) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Docstring

Fill colour row for a closed graphic (Rectangle / Circle) — the
shared [`color_field`] widget wired to the graphic-fill messages.
`picker` carries the transient open-state; the palette / HSV overlay
only expands when the picker targets this graphic's index.

## Source
Lines 173–206 in `crates/oxide-app/src/panels/symbol_editor_properties/graphic.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [graphic](/crates/oxide-app/src/panels/symbol_editor_properties/graphic.md) |
| calls | [color_field](/crates/oxide-app/src/panels/color_field/color_field.md) |
| called_by | [view_graphic_selection](/crates/oxide-app/src/panels/symbol_editor_properties/graphic/view_graphic_selection.md) |
