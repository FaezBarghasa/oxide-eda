---
okf_version: "0.2"
type: Function
title: render_pad_form_pad_features
description: "v0.20 — render the \"Pad Features\" section: top/bottom surface"
resource: crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/render_pad_form_pad_features
language: rust
---

# render_pad_form_pad_features

v0.20 — render the "Pad Features" section: top/bottom surface

## Signature

```rust
pub(in crate::panels::footprint_editor_properties) fn render_pad_form_pad_features(
    mut col: Column<'a, PanelMsg>,
    values: &PadFormValues,
    target: PadEditTarget,
    palette: PanelPalette,
    collapsed_sections: &'a CollapsedSections,
) -> Column<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(in crate::panels::footprint_editor_properties)`

## Docstring

v0.20 — render the "Pad Features" section: top/bottom surface
treatment + testpoint flags.

## Source
Lines 447–521 in `crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [form](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod.md) |
| calls | [props_section_header](/crates/oxide-app/src/panels/footprint_editor_properties/mod/props_section_header.md) |
| calls | [fp_is_collapsed](/crates/oxide-app/src/panels/footprint_editor_properties/mod/fp_is_collapsed.md) |
| calls | [pad_pick_row](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/pad_pick_row.md) |
| calls | [pad_check_row](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/pad_check_row.md) |
| called_by | [view_footprint_editor_properties](/crates/oxide-app/src/panels/footprint_editor_properties/mod/view_footprint_editor_properties.md) |
| called_by | [view_selection](/crates/oxide-app/src/panels/footprint_editor_properties/selection/view_selection.md) |
