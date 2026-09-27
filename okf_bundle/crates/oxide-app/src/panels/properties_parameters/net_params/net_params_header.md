---
okf_version: "0.2"
type: Function
title: net_params_header
description: Two-column Name / Value header for the Parameters (Net) table.
resource: crates/oxide-app/src/panels/properties_parameters/net_params.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/properties_parameters/net_params/net_params_header
language: rust
---

# net_params_header

Two-column Name / Value header for the Parameters (Net) table.

## Signature

```rust
pub fn net_params_header(label_c: Color, border_c: Color) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Two-column Name / Value header for the Parameters (Net) table.

## Source
Lines 101–126 in `crates/oxide-app/src/panels/properties_parameters/net_params.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [net_params](/crates/oxide-app/src/panels/properties_parameters/net_params.md) |
| called_by | [view_selected_element_properties](/crates/oxide-app/src/panels/element_properties/selected/view_selected_element_properties.md) |
