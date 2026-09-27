---
okf_version: "0.2"
type: Function
title: net_params_add_bar
description: Add / edit / delete toolbar at the bottom of the Parameters table.
resource: crates/oxide-app/src/panels/properties_parameters/net_params.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/properties_parameters/net_params/net_params_add_bar
language: rust
---

# net_params_add_bar

Add / edit / delete toolbar at the bottom of the Parameters table.

## Signature

```rust
pub fn net_params_add_bar(
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

Add / edit / delete toolbar at the bottom of the Parameters table.

## Source
Lines 153–203 in `crates/oxide-app/src/panels/properties_parameters/net_params.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [net_params](/crates/oxide-app/src/panels/properties_parameters/net_params.md) |
| called_by | [view_selected_element_properties](/crates/oxide-app/src/panels/element_properties/selected/view_selected_element_properties.md) |
