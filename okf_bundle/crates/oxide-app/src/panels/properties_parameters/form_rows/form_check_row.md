---
okf_version: "0.2"
type: Function
title: form_check_row
description: Checkbox form row.
resource: crates/oxide-app/src/panels/properties_parameters/form_rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/properties_parameters/form_rows/form_check_row
language: rust
---

# form_check_row

Checkbox form row.

## Signature

```rust
pub fn form_check_row(
    label: &str,
    checked: bool,
    msg: PanelMsg,
    label_c: Color,
) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Checkbox form row.

## Source
Lines 219–246 in `crates/oxide-app/src/panels/properties_parameters/form_rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [form_rows](/crates/oxide-app/src/panels/properties_parameters/form_rows.md) |
| called_by | [view_selected_element_properties](/crates/oxide-app/src/panels/element_properties/selected/view_selected_element_properties.md) |
| called_by | [render_cutout_subform](/crates/oxide-app/src/panels/footprint_editor_properties/subforms/cutout/render_cutout_subform.md) |
| called_by | [render_keepout_subform](/crates/oxide-app/src/panels/footprint_editor_properties/subforms/keepout/render_keepout_subform.md) |
