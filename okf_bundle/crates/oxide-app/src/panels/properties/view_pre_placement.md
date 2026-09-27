---
okf_version: "0.2"
type: Function
title: view_pre_placement
description: Pre-placement properties — shown when TAB pressed during a placement tool.
resource: crates/oxide-app/src/panels/properties.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/properties/view_pre_placement
language: rust
---

# view_pre_placement

Pre-placement properties — shown when TAB pressed during a placement tool.

## Signature

```rust
fn view_pre_placement(
    pp: &PrePlacementData,
    ctx: &'a PanelContext,
    muted: Color,
    primary: Color,
    border_c: Color,
    input_bg: Color,
    input_bdr: Color,
) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Docstring

Pre-placement properties — shown when TAB pressed during a placement tool.

## Source
Lines 285–498 in `crates/oxide-app/src/panels/properties.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [properties](/crates/oxide-app/src/panels/properties.md) |
| calls | [collapsible_section](/crates/oxide-app/src/panels/widgets/collapsible_section.md) |
| calls | [form_input_row](/crates/oxide-app/src/panels/properties_parameters/form_rows/form_input_row.md) |
| calls | [form_pick_row](/crates/oxide-app/src/panels/properties_parameters/form_rows/form_pick_row.md) |
| calls | [form_edit_row](/crates/oxide-app/src/panels/properties_parameters/form_rows/form_edit_row.md) |
| calls | [system_font_families](/crates/oxide-app/src/fonts/mod/system_font_families.md) |
| calls | [font_style_row](/crates/oxide-app/src/panels/properties_parameters/form_rows/font_style_row.md) |
| calls | [form_label](/crates/oxide-app/src/panels/properties_parameters/form_rows/form_label.md) |
| calls | [preplacement_justification_grid](/crates/oxide-app/src/panels/properties_parameters/net_params/preplacement_justification_grid.md) |
| calls | [form_edit_row_f64](/crates/oxide-app/src/panels/widgets/form_edit_row_f64.md) |
| calls | [shape_fill_row](/crates/oxide-app/src/panels/widgets/shape_fill_row.md) |
| called_by | [view_properties](/crates/oxide-app/src/panels/properties/view_properties.md) |
