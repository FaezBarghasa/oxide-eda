---
okf_version: "0.2"
type: Function
title: preplacement_justification_grid
description: Pre-placement 3x3 justification picker. Same visual grid as the
resource: crates/oxide-app/src/panels/properties_parameters/net_params.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/properties_parameters/net_params/preplacement_justification_grid
language: rust
---

# preplacement_justification_grid

Pre-placement 3x3 justification picker. Same visual grid as the

## Signature

```rust
pub fn preplacement_justification_grid(
    h: oxide_types::schematic::HAlign,
    input_bg: Color,
    input_bdr: Color,
    primary: Color,
    theme: oxide_types::theme::ThemeId,
) -> Element<'static, PanelMsg>
```

## Visibility

- `pub`

## Docstring

Pre-placement 3x3 justification picker. Same visual grid as the
selection-aware `justification_grid` but dispatches to the
`SetPrePlacementJustifyH` message family (no UUID needed).

## Source
Lines 379–494 in `crates/oxide-app/src/panels/properties_parameters/net_params.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [net_params](/crates/oxide-app/src/panels/properties_parameters/net_params.md) |
| called_by | [view_pre_placement](/crates/oxide-app/src/panels/properties/view_pre_placement.md) |
