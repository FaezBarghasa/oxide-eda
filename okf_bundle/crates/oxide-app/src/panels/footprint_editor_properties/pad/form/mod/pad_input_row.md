---
okf_version: "0.2"
type: Function
title: pad_input_row
description: v0.20 — single-line label + text-input row used by every Pad
resource: crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/pad_input_row
language: rust
---

# pad_input_row

v0.20 — single-line label + text-input row used by every Pad

## Signature

```rust
pub(in crate::panels::footprint_editor_properties) fn pad_input_row(
    label: &'a str,
    placeholder: &'a str,
    value: String,
    on_input: impl Fn(String) -> PanelMsg + 'a,
    muted: Color,
    primary: Color,
    border_c: Color,
) -> iced::Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(in crate::panels::footprint_editor_properties)`

## Docstring

v0.20 — single-line label + text-input row used by every Pad
Properties field. Mirrors the existing rotation/size_x rows'
chrome (40 px label, padded input, dim border).

## Source
Lines 214–254 in `crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [form](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod.md) |
| called_by | [render_pad_form_properties](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/render_pad_form_properties.md) |
| called_by | [render_pad_form_pad_stack](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/stack/render_pad_form_pad_stack.md) |
| called_by | [view_selection](/crates/oxide-app/src/panels/footprint_editor_properties/selection/view_selection.md) |
| called_by | [render_pattern_subform](/crates/oxide-app/src/panels/footprint_editor_properties/subforms/pattern/render_pattern_subform.md) |
| called_by | [render_sketch_pad_subform](/crates/oxide-app/src/panels/footprint_editor_properties/subforms/sketch_pad/render_sketch_pad_subform.md) |
