---
okf_version: "0.2"
type: Function
title: render_cutout_subform
description: v0.16.4 — BoardCutout role sub-form. Edge-radius expression input
resource: crates/oxide-app/src/panels/footprint_editor_properties/subforms/cutout.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/subforms/cutout/render_cutout_subform
language: rust
---

# render_cutout_subform

v0.16.4 — BoardCutout role sub-form. Edge-radius expression input

## Signature

```rust
pub(in crate::panels::footprint_editor_properties) fn render_cutout_subform(
    mut col: Column<'a, PanelMsg>,
    fp: &'a FootprintEditorPanelContext,
    id: oxide_sketch::id::SketchEntityId,
    muted: Color,
    primary: Color,
    border_c: Color,
) -> Column<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(in crate::panels::footprint_editor_properties)`

## Docstring

v0.16.4 — BoardCutout role sub-form. Edge-radius expression input
+ through-vs-partial-depth toggle.

## Source
Lines 10–72 in `crates/oxide-app/src/panels/footprint_editor_properties/subforms/cutout.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cutout](/crates/oxide-app/src/panels/footprint_editor_properties/subforms/cutout.md) |
| calls | [form_check_row](/crates/oxide-app/src/panels/properties_parameters/form_rows/form_check_row.md) |
| called_by | [view_selection](/crates/oxide-app/src/panels/footprint_editor_properties/selection/view_selection.md) |
