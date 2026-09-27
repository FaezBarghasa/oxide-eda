---
okf_version: "0.2"
type: Function
title: form_mm_edit_row
description: "Form row: label | floating-point mm text_input (no spinner buttons)."
resource: crates/oxide-app/src/panels/properties_parameters/form_rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/properties_parameters/form_rows/form_mm_edit_row
language: rust
---

# form_mm_edit_row

Form row: label | floating-point mm text_input (no spinner buttons).

## Signature

```rust
pub fn form_mm_edit_row(
    label: &str,
    value: f32,
    on_change: impl Fn(f32) -> PanelMsg + 'a + Clone,
    label_c: Color,
    input_bg: Color,
    input_border: Color,
) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Form row: label | floating-point mm text_input (no spinner buttons).

## Source
Lines 126–166 in `crates/oxide-app/src/panels/properties_parameters/form_rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [form_rows](/crates/oxide-app/src/panels/properties_parameters/form_rows.md) |
| called_by | [view_properties_general](/crates/oxide-app/src/panels/properties_parameters/general/view_properties_general.md) |
