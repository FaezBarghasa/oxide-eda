---
okf_version: "0.2"
type: Function
title: render_pour_subform
description: "v0.16.4 — Pour role sub-form. Renders when the entity's `pour`"
resource: crates/oxide-app/src/panels/footprint_editor_properties/subforms/pour.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/subforms/pour/render_pour_subform
language: rust
---

# render_pour_subform

v0.16.4 — Pour role sub-form. Renders when the entity's `pour`

## Signature

```rust
pub(in crate::panels::footprint_editor_properties) fn render_pour_subform(
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

v0.16.4 — Pour role sub-form. Renders when the entity's `pour`
attr is set; otherwise the column passes through unchanged.

## Source
Lines 10–118 in `crates/oxide-app/src/panels/footprint_editor_properties/subforms/pour.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pour](/crates/oxide-app/src/panels/footprint_editor_properties/subforms/pour.md) |
| called_by | [view_selection](/crates/oxide-app/src/panels/footprint_editor_properties/selection/view_selection.md) |
