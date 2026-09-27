---
okf_version: "0.2"
type: Function
title: view_properties_general
resource: crates/oxide-app/src/panels/properties_parameters/general.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/properties_parameters/general/view_properties_general
language: rust
---

# view_properties_general

## Signature

```rust
pub fn view_properties_general(
    ctx: &'a PanelContext,
    muted: Color,
    primary: Color,
    border_c: Color,
) -> Column<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Source
Lines 123–405 in `crates/oxide-app/src/panels/properties_parameters/general.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [general](/crates/oxide-app/src/panels/properties_parameters/general.md) |
| calls | [view_custom_selection_filters_section](/crates/oxide-app/src/panels/properties_parameters/general/view_custom_selection_filters_section.md) |
| calls | [collapsible_section](/crates/oxide-app/src/panels/widgets/collapsible_section.md) |
| calls | [form_label](/crates/oxide-app/src/panels/properties_parameters/form_rows/form_label.md) |
| calls | [form_grid_row](/crates/oxide-app/src/panels/properties_parameters/form_rows/form_grid_row.md) |
| calls | [form_check_row_shortcut](/crates/oxide-app/src/panels/properties_parameters/form_rows/form_check_row_shortcut.md) |
| calls | [form_font_link_row](/crates/oxide-app/src/panels/properties_parameters/form_rows/form_font_link_row.md) |
| calls | [canvas_font_popup](/crates/oxide-app/src/panels/properties_parameters/form_rows/canvas_font_popup.md) |
| calls | [form_pick_row](/crates/oxide-app/src/panels/properties_parameters/form_rows/form_pick_row.md) |
| calls | [paper_dimensions](/crates/oxide-app/src/panels/paper/paper_dimensions.md) |
| calls | [form_mm_edit_row](/crates/oxide-app/src/panels/properties_parameters/form_rows/form_mm_edit_row.md) |
| calls | [form_int_edit_row](/crates/oxide-app/src/panels/properties_parameters/form_rows/form_int_edit_row.md) |
| called_by | [view_properties](/crates/oxide-app/src/panels/properties/view_properties.md) |
