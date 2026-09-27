---
okf_version: "0.2"
type: Function
title: view_graphic_selection
description: Per-shape numeric Properties rows for a placed graphic (corners /
resource: crates/oxide-app/src/panels/symbol_editor_properties/graphic.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/symbol_editor_properties/graphic/view_graphic_selection
language: rust
---

# view_graphic_selection

Per-shape numeric Properties rows for a placed graphic (corners /

## Signature

```rust
pub(super) fn view_graphic_selection(
    mut col: Column<'a, PanelMsg>,
    g: &'a GraphicSummary,
    muted: Color,
    border_c: Color,
    fill_picker: Option<crate::app::GraphicFillPicker>,
) -> Column<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Docstring

Per-shape numeric Properties rows for a placed graphic (corners /
endpoints / centre + radius / arc angles / text + stroke).

## Source
Lines 12–167 in `crates/oxide-app/src/panels/symbol_editor_properties/graphic.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [graphic](/crates/oxide-app/src/panels/symbol_editor_properties/graphic.md) |
| calls | [graphic_fill_field](/crates/oxide-app/src/panels/symbol_editor_properties/graphic/graphic_fill_field.md) |
| called_by | [view_symbol_editor_properties](/crates/oxide-app/src/panels/symbol_editor_properties/mod/view_symbol_editor_properties.md) |
