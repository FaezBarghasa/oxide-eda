---
okf_version: "0.2"
type: Function
title: pad_pick_row
description: v0.20 — pick_list row for a Pad Properties field.
resource: crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/pad_pick_row
language: rust
---

# pad_pick_row

v0.20 — pick_list row for a Pad Properties field.

## Signature

```rust
pub(in crate::panels::footprint_editor_properties) fn pad_pick_row(
    label: &'a str,
    options: &'a [T],
    selected: T,
    on_change: impl Fn(T) -> PanelMsg + 'a + Clone,
    muted: Color,
) -> iced::Element<'a, PanelMsg>
```

## Type Parameters

- `'a`
- `T`

## Visibility

- `pub(in crate::panels::footprint_editor_properties)`

## Docstring

v0.20 — pick_list row for a Pad Properties field.

## Source
Lines 257–284 in `crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [form](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod.md) |
| called_by | [render_pad_form_pad_features](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/render_pad_form_pad_features.md) |
| called_by | [render_pad_form_properties](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/render_pad_form_properties.md) |
| called_by | [view_selection](/crates/oxide-app/src/panels/footprint_editor_properties/selection/view_selection.md) |
| called_by | [render_sketch_pad_subform](/crates/oxide-app/src/panels/footprint_editor_properties/subforms/sketch_pad/render_sketch_pad_subform.md) |
