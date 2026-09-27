---
okf_version: "0.2"
type: Function
title: child_sheet_stroke_width_row
description: Numeric stroke-width row for the child-sheet Style section.
resource: crates/oxide-app/src/panels/element_properties/child_sheet.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/element_properties/child_sheet/child_sheet_stroke_width_row
language: rust
---

# child_sheet_stroke_width_row

Numeric stroke-width row for the child-sheet Style section.

## Signature

```rust
fn child_sheet_stroke_width_row(
    sheet_id: uuid::Uuid,
    stored_value: f64,
    buffered: Option<String>,
    muted: Color,
    border_c: Color,
) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Docstring

Numeric stroke-width row for the child-sheet Style section.

## Source
Lines 193–240 in `crates/oxide-app/src/panels/element_properties/child_sheet.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [child_sheet](/crates/oxide-app/src/panels/element_properties/child_sheet.md) |
| called_by | [view_child_sheet_properties](/crates/oxide-app/src/panels/element_properties/child_sheet/view_child_sheet_properties.md) |
