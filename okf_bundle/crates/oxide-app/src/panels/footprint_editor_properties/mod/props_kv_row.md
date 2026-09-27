---
okf_version: "0.2"
type: Function
title: props_kv_row
description: Read-only key-value row — delegates to the schematic Properties
resource: crates/oxide-app/src/panels/footprint_editor_properties/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/mod/props_kv_row
language: rust
---

# props_kv_row

Read-only key-value row — delegates to the schematic Properties

## Signature

```rust
fn props_kv_row(
    col: Column<'a, PanelMsg>,
    label_c: Color,
    input_bg: Color,
    input_bdr: Color,
    key: &str,
    value: String,
) -> Column<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Docstring

Read-only key-value row — delegates to the schematic Properties
panel's `form_input_row` so the footprint editor uses identical
chrome (orange-accent border, dark-blue selection-tinted background).
Returns the updated Column to keep the chained-update call style
the rest of this module uses.

## Source
Lines 293–304 in `crates/oxide-app/src/panels/footprint_editor_properties/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_editor_properties](/crates/oxide-app/src/panels/footprint_editor_properties/mod.md) |
| calls | [form_input_row](/crates/oxide-app/src/panels/properties_parameters/form_rows/form_input_row.md) |
| called_by | [view_footprint_editor_properties](/crates/oxide-app/src/panels/footprint_editor_properties/mod/view_footprint_editor_properties.md) |
| called_by | [view_sections](/crates/oxide-app/src/panels/footprint_editor_properties/sections/view_sections.md) |
| called_by | [view_selection](/crates/oxide-app/src/panels/footprint_editor_properties/selection/view_selection.md) |
