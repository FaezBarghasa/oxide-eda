---
okf_version: "0.2"
type: Function
title: param_table_row
description: Parameter table row with subtle bottom border.
resource: crates/oxide-app/src/panels/properties_parameters/general.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/properties_parameters/general/param_table_row
language: rust
---

# param_table_row

Parameter table row with subtle bottom border.

## Signature

```rust
pub fn param_table_row(
    name: &str,
    value: &str,
    name_c: Color,
    val_c: Color,
    border_c: Color,
) -> Element<'a, M>
```

## Type Parameters

- `'a`
- `M: 'a`

## Visibility

- `pub`

## Docstring

Parameter table row with subtle bottom border.

## Source
Lines 512–540 in `crates/oxide-app/src/panels/properties_parameters/general.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [general](/crates/oxide-app/src/panels/properties_parameters/general.md) |
| called_by | [view_properties_parameters](/crates/oxide-app/src/panels/properties_parameters/general/view_properties_parameters.md) |
