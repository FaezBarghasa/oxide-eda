---
okf_version: "0.2"
type: Function
title: canvas_font_popup
resource: crates/oxide-app/src/panels/properties_parameters/form_rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/properties_parameters/form_rows/canvas_font_popup
language: rust
---

# canvas_font_popup

## Signature

```rust
pub fn canvas_font_popup(
    current_family: &str,
    current_size_px: f32,
    bold: bool,
    italic: bool,
    label_c: Color,
    input_bg: Color,
    input_bdr: Color,
) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Source
Lines 462–545 in `crates/oxide-app/src/panels/properties_parameters/form_rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [form_rows](/crates/oxide-app/src/panels/properties_parameters/form_rows.md) |
| calls | [system_font_families](/crates/oxide-app/src/fonts/mod/system_font_families.md) |
| called_by | [view_properties_general](/crates/oxide-app/src/panels/properties_parameters/general/view_properties_general.md) |
