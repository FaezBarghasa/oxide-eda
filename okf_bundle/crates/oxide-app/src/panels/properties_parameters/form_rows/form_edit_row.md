---
okf_version: "0.2"
type: Function
title: form_edit_row
description: "Form row: label | editable text_input that emits a PanelMsg on input."
resource: crates/oxide-app/src/panels/properties_parameters/form_rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/properties_parameters/form_rows/form_edit_row
language: rust
---

# form_edit_row

Form row: label | editable text_input that emits a PanelMsg on input.

## Signature

```rust
pub fn form_edit_row(
    label: &str,
    value: &str,
    label_c: Color,
    on_input: impl Fn(String) -> PanelMsg + 'a,
) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Form row: label | editable text_input that emits a PanelMsg on input.

## Source
Lines 587–608 in `crates/oxide-app/src/panels/properties_parameters/form_rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [form_rows](/crates/oxide-app/src/panels/properties_parameters/form_rows.md) |
| called_by | [view_selected_element_properties](/crates/oxide-app/src/panels/element_properties/selected/view_selected_element_properties.md) |
| called_by | [view_selection](/crates/oxide-app/src/panels/footprint_editor_properties/selection/view_selection.md) |
| called_by | [view_pre_placement](/crates/oxide-app/src/panels/properties/view_pre_placement.md) |
