---
okf_version: "0.2"
type: Function
title: view_selection
resource: crates/oxide-app/src/panels/footprint_editor_properties/selection.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/selection/view_selection
language: rust
---

# view_selection

## Signature

```rust
pub(super) fn view_selection(
    mut col: Column<'a, PanelMsg>,
    fp: &'a FootprintEditorPanelContext,
    mode_label: &str,
    palette: PanelPalette,
    custom_filter_presets: Vec<crate::active_bar::CustomFilterPreset>,
    active_custom_filter_tab: usize,
    collapsed_sections: &'a CollapsedSections,
) -> Column<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Source
Lines 18–506 in `crates/oxide-app/src/panels/footprint_editor_properties/selection.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [selection](/crates/oxide-app/src/panels/footprint_editor_properties/selection.md) |
| calls | [render_pad_form_properties](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/render_pad_form_properties.md) |
| calls | [props_kv_row](/crates/oxide-app/src/panels/footprint_editor_properties/mod/props_kv_row.md) |
| calls | [render_pad_form_pad_stack](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/stack/render_pad_form_pad_stack.md) |
| calls | [render_pad_form_pad_features](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/render_pad_form_pad_features.md) |
| calls | [props_section_header](/crates/oxide-app/src/panels/footprint_editor_properties/mod/props_section_header.md) |
| calls | [fp_is_collapsed](/crates/oxide-app/src/panels/footprint_editor_properties/mod/fp_is_collapsed.md) |
| calls | [render_pour_subform](/crates/oxide-app/src/panels/footprint_editor_properties/subforms/pour/render_pour_subform.md) |
| calls | [render_keepout_subform](/crates/oxide-app/src/panels/footprint_editor_properties/subforms/keepout/render_keepout_subform.md) |
| calls | [render_cutout_subform](/crates/oxide-app/src/panels/footprint_editor_properties/subforms/cutout/render_cutout_subform.md) |
| calls | [render_sketch_pad_subform](/crates/oxide-app/src/panels/footprint_editor_properties/subforms/sketch_pad/render_sketch_pad_subform.md) |
| calls | [render_pattern_subform](/crates/oxide-app/src/panels/footprint_editor_properties/subforms/pattern/render_pattern_subform.md) |
| calls | [pad_input_row](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/pad_input_row.md) |
| calls | [pad_check_row](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/pad_check_row.md) |
| calls | [view_custom_selection_filters_section](/crates/oxide-app/src/panels/properties_parameters/general/view_custom_selection_filters_section.md) |
| calls | [form_edit_row](/crates/oxide-app/src/panels/properties_parameters/form_rows/form_edit_row.md) |
| calls | [pad_pick_row](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/pad_pick_row.md) |
| called_by | [view_footprint_editor_properties](/crates/oxide-app/src/panels/footprint_editor_properties/mod/view_footprint_editor_properties.md) |
