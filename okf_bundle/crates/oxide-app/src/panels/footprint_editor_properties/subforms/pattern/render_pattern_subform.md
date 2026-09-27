---
okf_version: "0.2"
type: Function
title: render_pattern_subform
description: v0.23 — Pattern Properties sub-form. Renders the editable
resource: crates/oxide-app/src/panels/footprint_editor_properties/subforms/pattern.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/subforms/pattern/render_pattern_subform
language: rust
---

# render_pattern_subform

v0.23 — Pattern Properties sub-form. Renders the editable

## Signature

```rust
pub(in crate::panels::footprint_editor_properties) fn render_pattern_subform(
    mut col: Column<'a, PanelMsg>,
    fp: &'a FootprintEditorPanelContext,
    muted: Color,
    primary: Color,
    border_c: Color,
    collapsed_sections: &'a CollapsedSections,
) -> Column<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(in crate::panels::footprint_editor_properties)`

## Docstring

v0.23 — Pattern Properties sub-form. Renders the editable
expressions for a Linear / Grid / Polar [`oxide_sketch::array`]
when the selected sketch entity is its source. Each text input
emits a [`PanelMsg::FpEditorEditArrayParam`]; the numbering
pick_list emits [`PanelMsg::FpEditorSetArrayNumberingScheme`]; the
Delete button emits [`PanelMsg::FpEditorDeleteArray`]; the Re-pick
centre button (Polar only) emits
[`PanelMsg::FpEditorBeginRepickPolarCenter`].

## Source
Lines 23–412 in `crates/oxide-app/src/panels/footprint_editor_properties/subforms/pattern.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pattern](/crates/oxide-app/src/panels/footprint_editor_properties/subforms/pattern.md) |
| calls | [props_section_header](/crates/oxide-app/src/panels/footprint_editor_properties/mod/props_section_header.md) |
| calls | [fp_is_collapsed](/crates/oxide-app/src/panels/footprint_editor_properties/mod/fp_is_collapsed.md) |
| calls | [pad_input_row](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/pad_input_row.md) |
| calls | [pad_check_row](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/pad_check_row.md) |
| called_by | [view_selection](/crates/oxide-app/src/panels/footprint_editor_properties/selection/view_selection.md) |
