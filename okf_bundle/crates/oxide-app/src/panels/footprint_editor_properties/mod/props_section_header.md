---
okf_version: "0.2"
type: Function
title: props_section_header
description: Section header — collapsible. Delegates to
resource: crates/oxide-app/src/panels/footprint_editor_properties/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/mod/props_section_header
language: rust
---

# props_section_header

Section header — collapsible. Delegates to

## Signature

```rust
pub(super) fn props_section_header(
    label: &str,
    key: &'static str,
    collapsed: &super::CollapsedSections,
    primary: Color,
    border_c: Color,
) -> iced::widget::Column<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Docstring

Section header — collapsible. Delegates to
`super::collapsible_section_header` so every footprint Properties
section gets the same clickable chevron header used by the
schematic's Custom Selection Filters / General sections. Each
call site supplies a unique `key` so collapsed state survives in
`PanelContext.collapsed_sections`. Callers guard their body push
with `if !is_section_collapsed(key, collapsed)`.

## Source
Lines 272–280 in `crates/oxide-app/src/panels/footprint_editor_properties/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_editor_properties](/crates/oxide-app/src/panels/footprint_editor_properties/mod.md) |
| calls | [collapsible_section_header](/crates/oxide-app/src/panels/widgets/collapsible_section_header.md) |
| called_by | [render_fp_settings_and_hint](/crates/oxide-app/src/panels/footprint_editor_properties/mod/render_fp_settings_and_hint.md) |
| called_by | [render_pad_form_pad_features](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/render_pad_form_pad_features.md) |
| called_by | [render_pad_form_properties](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/render_pad_form_properties.md) |
| called_by | [render_pad_form_pad_stack](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/stack/render_pad_form_pad_stack.md) |
| called_by | [view_sections](/crates/oxide-app/src/panels/footprint_editor_properties/sections/view_sections.md) |
| called_by | [view_selection](/crates/oxide-app/src/panels/footprint_editor_properties/selection/view_selection.md) |
| called_by | [render_pattern_subform](/crates/oxide-app/src/panels/footprint_editor_properties/subforms/pattern/render_pattern_subform.md) |
| called_by | [render_sketch_pad_subform](/crates/oxide-app/src/panels/footprint_editor_properties/subforms/sketch_pad/render_sketch_pad_subform.md) |
