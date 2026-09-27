---
okf_version: "0.2"
type: Function
title: pad_copper_row
description: v0.20 — single COPPER table row. Built inline because all four
resource: crates/oxide-app/src/panels/footprint_editor_properties/pad/table.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/pad/table/pad_copper_row
language: rust
---

# pad_copper_row

v0.20 — single COPPER table row. Built inline because all four

## Signature

```rust
pub(super) fn pad_copper_row(
    label: &'a str,
    values: &PadFormValues,
    current_shape: PadShapeChoice,
    target: PadEditTarget,
    palette: PanelPalette,
    is_authoritative: bool,
) -> iced::Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Docstring

v0.20 — single COPPER table row. Built inline because all four
data cells (X-Size, Y-Size, Shape, Relief) reference different
fields on PadFormValues + different message constructors.

## Source
Lines 185–252 in `crates/oxide-app/src/panels/footprint_editor_properties/pad/table.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [table](/crates/oxide-app/src/panels/footprint_editor_properties/pad/table.md) |
| calls | [pad_table_row](/crates/oxide-app/src/panels/footprint_editor_properties/pad/table/pad_table_row.md) |
| called_by | [render_pad_form_pad_stack](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/stack/render_pad_form_pad_stack.md) |
