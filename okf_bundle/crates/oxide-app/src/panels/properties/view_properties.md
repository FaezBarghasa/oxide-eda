---
okf_version: "0.2"
type: Function
title: view_properties
resource: crates/oxide-app/src/panels/properties.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/properties/view_properties
language: rust
---

# view_properties

## Signature

```rust
pub fn view_properties(ctx: &'a PanelContext) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Source
Lines 99–282 in `crates/oxide-app/src/panels/properties.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [properties](/crates/oxide-app/src/panels/properties.md) |
| calls | [view_library_row_properties](/crates/oxide-app/src/panels/library/view_library_row_properties.md) |
| calls | [view_symbol_editor_properties](/crates/oxide-app/src/panels/symbol_editor_properties/mod/view_symbol_editor_properties.md) |
| calls | [view_footprint_editor_properties](/crates/oxide-app/src/panels/footprint_editor_properties/mod/view_footprint_editor_properties.md) |
| calls | [view_pre_placement](/crates/oxide-app/src/panels/properties/view_pre_placement.md) |
| calls | [view_selected_element_properties](/crates/oxide-app/src/panels/element_properties/selected/view_selected_element_properties.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [view_properties_general](/crates/oxide-app/src/panels/properties_parameters/general/view_properties_general.md) |
| calls | [view_properties_parameters](/crates/oxide-app/src/panels/properties_parameters/general/view_properties_parameters.md) |
| called_by | [view_panel](/crates/oxide-app/src/panels/mod/view_panel.md) |
