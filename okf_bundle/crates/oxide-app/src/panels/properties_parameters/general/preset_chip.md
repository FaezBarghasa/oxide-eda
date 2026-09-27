---
okf_version: "0.2"
type: Function
title: preset_chip
description: Member chip for a custom-filter preset card (Properties panel).
resource: crates/oxide-app/src/panels/properties_parameters/general.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/properties_parameters/general/preset_chip
language: rust
---

# preset_chip

Member chip for a custom-filter preset card (Properties panel).

## Signature

```rust
pub fn preset_chip(
    label: &str,
    preset_idx: usize,
    filter: crate::active_bar::SelectionFilter,
    enabled: bool,
    hover_bg: Color,
    border_c: Color,
) -> Element<'static, PanelMsg>
```

## Visibility

- `pub`

## Docstring

Member chip for a custom-filter preset card (Properties panel).
Border colour matches the Active Bar Filter dropdown chips (theme
accent), so chip styling stays consistent across both surfaces.

## Source
Lines 619–656 in `crates/oxide-app/src/panels/properties_parameters/general.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [general](/crates/oxide-app/src/panels/properties_parameters/general.md) |
| called_by | [view_custom_selection_filters_section](/crates/oxide-app/src/panels/properties_parameters/general/view_custom_selection_filters_section.md) |
