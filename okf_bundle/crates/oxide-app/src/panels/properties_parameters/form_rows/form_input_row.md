---
okf_version: "0.2"
type: Function
title: form_input_row
description: "Form row: label | styled input-like value display."
resource: crates/oxide-app/src/panels/properties_parameters/form_rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/properties_parameters/form_rows/form_input_row
language: rust
---

# form_input_row

Form row: label | styled input-like value display.

## Signature

```rust
pub fn form_input_row(
    label: &str,
    value: &str,
    label_c: Color,
    input_bg: Color,
    input_border: Color,
) -> Element<'a, M>
```

## Type Parameters

- `'a`
- `M: 'a`

## Visibility

- `pub`

## Docstring

Form row: label | styled input-like value display.

## Source
Lines 42–80 in `crates/oxide-app/src/panels/properties_parameters/form_rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [form_rows](/crates/oxide-app/src/panels/properties_parameters/form_rows.md) |
| called_by | [view_components](/crates/oxide-app/src/panels/components/view_components.md) |
| called_by | [view_selected_element_properties](/crates/oxide-app/src/panels/element_properties/selected/view_selected_element_properties.md) |
| called_by | [props_kv_row](/crates/oxide-app/src/panels/footprint_editor_properties/mod/props_kv_row.md) |
| called_by | [view_pre_placement](/crates/oxide-app/src/panels/properties/view_pre_placement.md) |
