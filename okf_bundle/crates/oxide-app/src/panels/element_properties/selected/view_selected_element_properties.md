---
okf_version: "0.2"
type: Function
title: view_selected_element_properties
resource: crates/oxide-app/src/panels/element_properties/selected.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/element_properties/selected/view_selected_element_properties
language: rust
---

# view_selected_element_properties

## Signature

```rust
pub(in crate::panels) fn view_selected_element_properties(
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

## Source
Lines 10–793 in `crates/oxide-app/src/panels/element_properties/selected.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [selected](/crates/oxide-app/src/panels/element_properties/selected.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [collapsible_section](/crates/oxide-app/src/panels/widgets/collapsible_section.md) |
| calls | [form_input_row](/crates/oxide-app/src/panels/properties_parameters/form_rows/form_input_row.md) |
| calls | [form_pick_row](/crates/oxide-app/src/panels/properties_parameters/form_rows/form_pick_row.md) |
| calls | [form_edit_row](/crates/oxide-app/src/panels/properties_parameters/form_rows/form_edit_row.md) |
| calls | [font_style_row](/crates/oxide-app/src/panels/properties_parameters/form_rows/font_style_row.md) |
| calls | [net_numeric_row](/crates/oxide-app/src/panels/properties_parameters/net_params/net_numeric_row.md) |
| calls | [net_params_tabs](/crates/oxide-app/src/panels/properties_parameters/net_params/net_params_tabs.md) |
| calls | [net_params_header](/crates/oxide-app/src/panels/properties_parameters/net_params/net_params_header.md) |
| calls | [empty_section_row](/crates/oxide-app/src/panels/properties_parameters/net_params/empty_section_row.md) |
| calls | [net_params_add_bar](/crates/oxide-app/src/panels/properties_parameters/net_params/net_params_add_bar.md) |
| calls | [form_check_row](/crates/oxide-app/src/panels/properties_parameters/form_rows/form_check_row.md) |
| calls | [expand_char_escapes](/crates/oxide-app/src/schematic_runtime/text/expand_char_escapes.md) |
| calls | [system_font_families](/crates/oxide-app/src/fonts/mod/system_font_families.md) |
| calls | [form_label](/crates/oxide-app/src/panels/properties_parameters/form_rows/form_label.md) |
| calls | [justification_grid](/crates/oxide-app/src/panels/properties_parameters/net_params/justification_grid.md) |
| calls | [view_drawing_properties](/crates/oxide-app/src/panels/element_properties/drawing/view_drawing_properties.md) |
| calls | [view_child_sheet_properties](/crates/oxide-app/src/panels/element_properties/child_sheet/view_child_sheet_properties.md) |
| calls | [prop_kv_row](/crates/oxide-app/src/panels/widgets/prop_kv_row.md) |
| called_by | [view_properties](/crates/oxide-app/src/panels/properties/view_properties.md) |
