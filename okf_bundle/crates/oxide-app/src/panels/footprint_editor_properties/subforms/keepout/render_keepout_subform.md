---
okf_version: "0.2"
type: Function
title: render_keepout_subform
description: v0.16.4 — Keepout role sub-form. Renders the 6 kind flags as a
resource: crates/oxide-app/src/panels/footprint_editor_properties/subforms/keepout.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/subforms/keepout/render_keepout_subform
language: rust
---

# render_keepout_subform

v0.16.4 — Keepout role sub-form. Renders the 6 kind flags as a

## Signature

```rust
pub(in crate::panels::footprint_editor_properties) fn render_keepout_subform(
    mut col: Column<'a, PanelMsg>,
    fp: &'a FootprintEditorPanelContext,
    id: oxide_sketch::id::SketchEntityId,
    muted: Color,
    primary: Color,
    _border_c: Color,
) -> Column<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(in crate::panels::footprint_editor_properties)`

## Docstring

v0.16.4 — Keepout role sub-form. Renders the 6 kind flags as a
vertical checklist when the entity's `keepout` attr is set.

## Source
Lines 10–92 in `crates/oxide-app/src/panels/footprint_editor_properties/subforms/keepout.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keepout](/crates/oxide-app/src/panels/footprint_editor_properties/subforms/keepout.md) |
| calls | [form_check_row](/crates/oxide-app/src/panels/properties_parameters/form_rows/form_check_row.md) |
| called_by | [view_selection](/crates/oxide-app/src/panels/footprint_editor_properties/selection/view_selection.md) |
