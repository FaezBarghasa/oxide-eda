---
okf_version: "0.2"
type: Function
title: empty_section_row
description: Empty-state row — centered muted text spanning the whole row.
resource: crates/oxide-app/src/panels/properties_parameters/net_params.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/properties_parameters/net_params/empty_section_row
language: rust
---

# empty_section_row

Empty-state row — centered muted text spanning the whole row.

## Signature

```rust
pub fn empty_section_row(
    label: &str,
    label_c: Color,
    border_c: Color,
) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Empty-state row — centered muted text spanning the whole row.

## Source
Lines 129–150 in `crates/oxide-app/src/panels/properties_parameters/net_params.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [net_params](/crates/oxide-app/src/panels/properties_parameters/net_params.md) |
| called_by | [view_selected_element_properties](/crates/oxide-app/src/panels/element_properties/selected/view_selected_element_properties.md) |
