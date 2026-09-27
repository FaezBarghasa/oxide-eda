---
okf_version: "0.2"
type: Function
title: net_numeric_row
description: "Net-attribute row: label | checkbox | text value | unit. Used for"
resource: crates/oxide-app/src/panels/properties_parameters/net_params.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/properties_parameters/net_params/net_numeric_row
language: rust
---

# net_numeric_row

Net-attribute row: label | checkbox | text value | unit. Used for

## Signature

```rust
pub fn net_numeric_row(
    label: &str,
    value: &str,
    unit: &str,
    label_c: Color,
    input_bg: Color,
    input_bdr: Color,
) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Net-attribute row: label | checkbox | text value | unit. Used for
"Power Net = 0.000 V" and "High Speed = 0.000 Hz".

## Source
Lines 10–45 in `crates/oxide-app/src/panels/properties_parameters/net_params.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [net_params](/crates/oxide-app/src/panels/properties_parameters/net_params.md) |
| called_by | [view_selected_element_properties](/crates/oxide-app/src/panels/element_properties/selected/view_selected_element_properties.md) |
