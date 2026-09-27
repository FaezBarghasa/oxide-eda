---
okf_version: "0.2"
type: Function
title: child_sheet_color_row
description: One swatch row in the child-sheet Style section.
resource: crates/oxide-app/src/panels/element_properties/child_sheet.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/element_properties/child_sheet/child_sheet_color_row
language: rust
---

# child_sheet_color_row

One swatch row in the child-sheet Style section.

## Signature

```rust
fn child_sheet_color_row(
    label: &'a str,
    sheet_id: uuid::Uuid,
    current: Option<oxide_types::schematic::StrokeColor>,
    show_picker: bool,
    show_advanced: bool,
    palette: PanelPalette,
    is_border: bool,
) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Docstring

One swatch row in the child-sheet Style section.

Click flow:
1. Click swatch → expands an inline preset palette panel below
the row (full panel width, like the canvas-font popup) with
a 12-colour grid plus "Custom…" and (when an override is
active) "Reset to Default".
2. Click the swatch again to collapse, or click "Custom…" to
switch to the iced_aw HSV / RGB ColorPicker overlay.

Both the palette pick and the advanced-picker submit reuse the
same `EditChildSheet*Color` message so engine command + undo/redo
round-trip is identical for both paths.

## Source
Lines 129–190 in `crates/oxide-app/src/panels/element_properties/child_sheet.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [child_sheet](/crates/oxide-app/src/panels/element_properties/child_sheet.md) |
| calls | [color_field](/crates/oxide-app/src/panels/color_field/color_field.md) |
| called_by | [view_child_sheet_properties](/crates/oxide-app/src/panels/element_properties/child_sheet/view_child_sheet_properties.md) |
