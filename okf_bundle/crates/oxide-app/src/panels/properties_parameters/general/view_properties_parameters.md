---
okf_version: "0.2"
type: Function
title: view_properties_parameters
resource: crates/oxide-app/src/panels/properties_parameters/general.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/properties_parameters/general/view_properties_parameters
language: rust
---

# view_properties_parameters

## Signature

```rust
pub fn view_properties_parameters(
    muted: Color,
    primary: Color,
    border_c: Color,
    input_bg: Color,
    input_bdr: Color,
    seg_hover: Color,
) -> Column<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Source
Lines 407–509 in `crates/oxide-app/src/panels/properties_parameters/general.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [general](/crates/oxide-app/src/panels/properties_parameters/general.md) |
| calls | [section_hdr](/crates/oxide-app/src/panels/properties_parameters/form_rows/section_hdr.md) |
| calls | [param_table_row](/crates/oxide-app/src/panels/properties_parameters/general/param_table_row.md) |
| called_by | [view_properties](/crates/oxide-app/src/panels/properties/view_properties.md) |
