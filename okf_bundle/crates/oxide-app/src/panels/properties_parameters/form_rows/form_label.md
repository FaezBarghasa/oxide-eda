---
okf_version: "0.2"
type: Function
title: form_label
description: "Standalone label row (no value, used before segmented controls)."
resource: crates/oxide-app/src/panels/properties_parameters/form_rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/properties_parameters/form_rows/form_label
language: rust
---

# form_label

Standalone label row (no value, used before segmented controls).

## Signature

```rust
pub fn form_label(label: &str, label_c: Color) -> Element<'a, M>
```

## Type Parameters

- `'a`
- `M: 'a`

## Visibility

- `pub`

## Docstring

Standalone label row (no value, used before segmented controls).

## Source
Lines 611–616 in `crates/oxide-app/src/panels/properties_parameters/form_rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [form_rows](/crates/oxide-app/src/panels/properties_parameters/form_rows.md) |
| called_by | [view_selected_element_properties](/crates/oxide-app/src/panels/element_properties/selected/view_selected_element_properties.md) |
| called_by | [render_other_section](/crates/oxide-app/src/panels/footprint_editor_properties/managers/render_other_section.md) |
| called_by | [view_pre_placement](/crates/oxide-app/src/panels/properties/view_pre_placement.md) |
| called_by | [view_properties_general](/crates/oxide-app/src/panels/properties_parameters/general/view_properties_general.md) |
