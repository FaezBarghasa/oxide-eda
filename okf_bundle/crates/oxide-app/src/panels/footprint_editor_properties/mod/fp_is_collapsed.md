---
okf_version: "0.2"
type: Function
title: fp_is_collapsed
description: "Returns true if the section with `key` is collapsed in"
resource: crates/oxide-app/src/panels/footprint_editor_properties/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/mod/fp_is_collapsed
language: rust
---

# fp_is_collapsed

Returns true if the section with `key` is collapsed in

## Signature

```rust
pub(super) fn fp_is_collapsed(key: &str, collapsed: &super::CollapsedSections) -> bool
```

## Visibility

- `pub(super)`

## Docstring

Returns true if the section with `key` is collapsed in
`PanelContext.collapsed_sections`.

## Source
Lines 284–286 in `crates/oxide-app/src/panels/footprint_editor_properties/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_editor_properties](/crates/oxide-app/src/panels/footprint_editor_properties/mod.md) |
| calls | [is_section_collapsed](/crates/oxide-app/src/panels/widgets/is_section_collapsed.md) |
| called_by | [render_fp_settings_and_hint](/crates/oxide-app/src/panels/footprint_editor_properties/mod/render_fp_settings_and_hint.md) |
| called_by | [render_pad_form_pad_features](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/render_pad_form_pad_features.md) |
| called_by | [render_pad_form_properties](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/render_pad_form_properties.md) |
| called_by | [render_pad_form_pad_stack](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/stack/render_pad_form_pad_stack.md) |
| called_by | [view_sections](/crates/oxide-app/src/panels/footprint_editor_properties/sections/view_sections.md) |
| called_by | [view_selection](/crates/oxide-app/src/panels/footprint_editor_properties/selection/view_selection.md) |
| called_by | [render_pattern_subform](/crates/oxide-app/src/panels/footprint_editor_properties/subforms/pattern/render_pattern_subform.md) |
| called_by | [render_sketch_pad_subform](/crates/oxide-app/src/panels/footprint_editor_properties/subforms/sketch_pad/render_sketch_pad_subform.md) |
