---
okf_version: "0.2"
type: Function
title: view_custom_selection_filters_section
description: Custom Selection Filters collapsible section — tabbed editor for up to
resource: crates/oxide-app/src/panels/properties_parameters/general.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/properties_parameters/general/view_custom_selection_filters_section
language: rust
---

# view_custom_selection_filters_section

Custom Selection Filters collapsible section — tabbed editor for up to

## Signature

```rust
pub fn view_custom_selection_filters_section(
    presets: Vec<crate::active_bar::CustomFilterPreset>,
    active_custom_filter_tab: usize,
    collapsed_sections: &'a CollapsedSections,
    palette: PanelPalette,
) -> Column<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Custom Selection Filters collapsible section — tabbed editor for up to
`CUSTOM_FILTER_PRESET_LIMIT` named presets. Pulled out of
`view_properties_general` so the schematic Properties panel and the
Footprint editor's Properties panel render the EXACT same widget.

## Source
Lines 15–121 in `crates/oxide-app/src/panels/properties_parameters/general.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [general](/crates/oxide-app/src/panels/properties_parameters/general.md) |
| calls | [collapsible_section](/crates/oxide-app/src/panels/widgets/collapsible_section.md) |
| calls | [custom_filter_tab](/crates/oxide-app/src/panels/properties_parameters/general/custom_filter_tab.md) |
| calls | [preset_chip](/crates/oxide-app/src/panels/properties_parameters/general/preset_chip.md) |
| called_by | [view_selection](/crates/oxide-app/src/panels/footprint_editor_properties/selection/view_selection.md) |
| called_by | [view_properties_general](/crates/oxide-app/src/panels/properties_parameters/general/view_properties_general.md) |
