---
okf_version: "0.2"
type: Function
title: collapsible_section
description: "Collapsible section: clickable header with SVG chevron, hides content when collapsed."
resource: crates/oxide-app/src/panels/widgets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/widgets/collapsible_section
language: rust
---

# collapsible_section

Collapsible section: clickable header with SVG chevron, hides content when collapsed.

## Signature

```rust
pub fn collapsible_section(
    key: &str,
    title: &str,
    collapsed: &CollapsedSections,
    header_color: Color,
    border_c: Color,
    content: impl FnOnce() -> Column<'a, PanelMsg>,
) -> Column<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Collapsible section: clickable header with SVG chevron, hides content when collapsed.

## Source
Lines 100–114 in `crates/oxide-app/src/panels/widgets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [widgets](/crates/oxide-app/src/panels/widgets.md) |
| calls | [collapsible_section_header](/crates/oxide-app/src/panels/widgets/collapsible_section_header.md) |
| called_by | [view_child_sheet_properties](/crates/oxide-app/src/panels/element_properties/child_sheet/view_child_sheet_properties.md) |
| called_by | [view_drawing_properties](/crates/oxide-app/src/panels/element_properties/drawing/view_drawing_properties.md) |
| called_by | [view_selected_element_properties](/crates/oxide-app/src/panels/element_properties/selected/view_selected_element_properties.md) |
| called_by | [view_pre_placement](/crates/oxide-app/src/panels/properties/view_pre_placement.md) |
| called_by | [view_custom_selection_filters_section](/crates/oxide-app/src/panels/properties_parameters/general/view_custom_selection_filters_section.md) |
| called_by | [view_properties_general](/crates/oxide-app/src/panels/properties_parameters/general/view_properties_general.md) |
