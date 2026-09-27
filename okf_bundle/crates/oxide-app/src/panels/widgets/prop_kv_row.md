---
okf_version: "0.2"
type: Function
title: prop_kv_row
description: Property key-value row (owned strings to avoid lifetime issues in closures).
resource: crates/oxide-app/src/panels/widgets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/widgets/prop_kv_row
language: rust
---

# prop_kv_row

Property key-value row (owned strings to avoid lifetime issues in closures).

## Signature

```rust
pub fn prop_kv_row(
    key: &str,
    value: &str,
    key_c: Color,
    val_c: Color,
) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Property key-value row (owned strings to avoid lifetime issues in closures).

## Source
Lines 117–133 in `crates/oxide-app/src/panels/widgets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [widgets](/crates/oxide-app/src/panels/widgets.md) |
| called_by | [view_child_sheet_properties](/crates/oxide-app/src/panels/element_properties/child_sheet/view_child_sheet_properties.md) |
| called_by | [view_drawing_properties](/crates/oxide-app/src/panels/element_properties/drawing/view_drawing_properties.md) |
| called_by | [view_selected_element_properties](/crates/oxide-app/src/panels/element_properties/selected/view_selected_element_properties.md) |
