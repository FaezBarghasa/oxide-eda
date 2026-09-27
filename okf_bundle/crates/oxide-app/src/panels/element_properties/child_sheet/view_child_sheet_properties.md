---
okf_version: "0.2"
type: Function
title: view_child_sheet_properties
description: Properties section for a single hierarchical child sheet.
resource: crates/oxide-app/src/panels/element_properties/child_sheet.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/element_properties/child_sheet/view_child_sheet_properties
language: rust
---

# view_child_sheet_properties

Properties section for a single hierarchical child sheet.

## Signature

```rust
pub(in crate::panels) fn view_child_sheet_properties(
    ctx: &'a PanelContext,
    muted: Color,
    primary: Color,
    border_c: Color,
) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(in crate::panels)`

## Docstring

Properties section for a single hierarchical child sheet.
Shows read-only info (Name / File / Position / Size) plus
editable Border Colour, Fill Colour and Line Width with a
Reset-to-default button. Colour edits open an iced_aw
ColorPicker overlay anchored to a swatch button.

## Source
Lines 13–114 in `crates/oxide-app/src/panels/element_properties/child_sheet.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [child_sheet](/crates/oxide-app/src/panels/element_properties/child_sheet.md) |
| calls | [collapsible_section](/crates/oxide-app/src/panels/widgets/collapsible_section.md) |
| calls | [prop_kv_row](/crates/oxide-app/src/panels/widgets/prop_kv_row.md) |
| calls | [child_sheet_color_row](/crates/oxide-app/src/panels/element_properties/child_sheet/child_sheet_color_row.md) |
| calls | [child_sheet_stroke_width_row](/crates/oxide-app/src/panels/element_properties/child_sheet/child_sheet_stroke_width_row.md) |
| called_by | [view_selected_element_properties](/crates/oxide-app/src/panels/element_properties/selected/view_selected_element_properties.md) |
