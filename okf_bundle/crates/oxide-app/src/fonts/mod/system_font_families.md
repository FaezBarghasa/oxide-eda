---
okf_version: "0.2"
type: Function
title: system_font_families
description: Return the list of distinct font family names available on this system.
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod/system_font_families
language: rust
---

# system_font_families

Return the list of distinct font family names available on this system.

## Signature

```rust
pub fn system_font_families() -> &'static Vec<String>
```

## Visibility

- `pub`

## Docstring

Return the list of distinct font family names available on this system.

Expensive on first call (scans system font directories via fontdb),
then cached for the lifetime of the process.

## Source
Lines 162–182 in `crates/oxide-app/src/fonts/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fonts](/crates/oxide-app/src/fonts/mod.md) |
| called_by | [view_selected_element_properties](/crates/oxide-app/src/panels/element_properties/selected/view_selected_element_properties.md) |
| called_by | [view_pre_placement](/crates/oxide-app/src/panels/properties/view_pre_placement.md) |
| called_by | [canvas_font_popup](/crates/oxide-app/src/panels/properties_parameters/form_rows/canvas_font_popup.md) |
