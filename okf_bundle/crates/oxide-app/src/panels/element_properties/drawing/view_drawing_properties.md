---
okf_version: "0.2"
type: Function
title: view_drawing_properties
resource: crates/oxide-app/src/panels/element_properties/drawing.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/element_properties/drawing/view_drawing_properties
language: rust
---

# view_drawing_properties

## Signature

```rust
pub(in crate::panels) fn view_drawing_properties(
    ctx: &'a PanelContext,
    muted: Color,
    _primary: Color,
    border_c: Color,
) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(in crate::panels)`

## Source
Lines 9–382 in `crates/oxide-app/src/panels/element_properties/drawing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [drawing](/crates/oxide-app/src/panels/element_properties/drawing.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [shape_icon_handle](/crates/oxide-app/src/panels/widgets/shape_icon_handle.md) |
| calls | [collapsible_section](/crates/oxide-app/src/panels/widgets/collapsible_section.md) |
| calls | [drawing_num_row](/crates/oxide-app/src/panels/element_properties/drawing/drawing_num_row.md) |
| calls | [drawing_fill_row](/crates/oxide-app/src/panels/element_properties/drawing/drawing_fill_row.md) |
| calls | [prop_kv_row](/crates/oxide-app/src/panels/widgets/prop_kv_row.md) |
| calls | [drawing_stroke_color_row](/crates/oxide-app/src/panels/element_properties/drawing/drawing_stroke_color_row.md) |
| called_by | [view_selected_element_properties](/crates/oxide-app/src/panels/element_properties/selected/view_selected_element_properties.md) |
