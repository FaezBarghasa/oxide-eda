---
okf_version: "0.2"
type: Function
title: justification_grid
description: Altium-style 3x3 justification picker with proper SVG arrow icons.
resource: crates/oxide-app/src/panels/properties_parameters/net_params.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/properties_parameters/net_params/justification_grid
language: rust
---

# justification_grid

Altium-style 3x3 justification picker with proper SVG arrow icons.

## Signature

```rust
pub fn justification_grid(
    id: uuid::Uuid,
    rotation_deg: f64,
    h: oxide_types::schematic::HAlign,
    palette: PanelPalette,
    theme: oxide_types::theme::ThemeId,
) -> Element<'static, PanelMsg>
```

## Visibility

- `pub`

## Docstring

Altium-style 3x3 justification picker with proper SVG arrow icons.
Only horizontal is wired to state for now; vertical slots toggle visually
but don't mutate the label.

## Source
Lines 208–374 in `crates/oxide-app/src/panels/properties_parameters/net_params.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [net_params](/crates/oxide-app/src/panels/properties_parameters/net_params.md) |
| called_by | [view_selected_element_properties](/crates/oxide-app/src/panels/element_properties/selected/view_selected_element_properties.md) |
