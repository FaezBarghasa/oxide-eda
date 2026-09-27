---
okf_version: "0.2"
type: Function
title: drawing_fill_row
resource: crates/oxide-app/src/panels/element_properties/drawing.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/element_properties/drawing/drawing_fill_row
language: rust
---

# drawing_fill_row

## Signature

```rust
fn drawing_fill_row(
    current: oxide_types::schematic::FillType,
    muted: Color,
    _border_c: Color,
) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Source
Lines 512–564 in `crates/oxide-app/src/panels/element_properties/drawing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [drawing](/crates/oxide-app/src/panels/element_properties/drawing.md) |
| called_by | [view_drawing_properties](/crates/oxide-app/src/panels/element_properties/drawing/view_drawing_properties.md) |
