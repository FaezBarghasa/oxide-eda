---
okf_version: "0.2"
type: Function
title: custom_filter_tab
description: Tab button in the Custom Selection Filters tab strip. Active tab
resource: crates/oxide-app/src/panels/properties_parameters/general.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/properties_parameters/general/custom_filter_tab
language: rust
---

# custom_filter_tab

Tab button in the Custom Selection Filters tab strip. Active tab

## Signature

```rust
pub fn custom_filter_tab(
    label: String,
    active: bool,
    idx: usize,
    hover_bg: Color,
    border_c: Color,
) -> Element<'static, PanelMsg>
```

## Visibility

- `pub`

## Docstring

Tab button in the Custom Selection Filters tab strip. Active tab
gets a filled background; both states use the theme accent for the
border so the section reads as one piece with the chips and the
Active Bar dropdown.

## Source
Lines 579–614 in `crates/oxide-app/src/panels/properties_parameters/general.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [general](/crates/oxide-app/src/panels/properties_parameters/general.md) |
| called_by | [view_custom_selection_filters_section](/crates/oxide-app/src/panels/properties_parameters/general/view_custom_selection_filters_section.md) |
