---
okf_version: "0.2"
type: Function
title: form_font_link_row
resource: crates/oxide-app/src/panels/properties_parameters/form_rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/properties_parameters/form_rows/form_font_link_row
language: rust
---

# form_font_link_row

## Signature

```rust
pub fn form_font_link_row(
    label: &'static str,
    current_family: &str,
    current_size_px: f32,
    _bold: bool,
    _italic: bool,
    label_c: Color,
) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Source
Lines 412–460 in `crates/oxide-app/src/panels/properties_parameters/form_rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [form_rows](/crates/oxide-app/src/panels/properties_parameters/form_rows.md) |
| called_by | [view_properties_general](/crates/oxide-app/src/panels/properties_parameters/general/view_properties_general.md) |
