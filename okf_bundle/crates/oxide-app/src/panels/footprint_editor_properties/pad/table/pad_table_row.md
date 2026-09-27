---
okf_version: "0.2"
type: Function
title: pad_table_row
description: v0.20 — Altium-style table data row. First cell is the row label
resource: crates/oxide-app/src/panels/footprint_editor_properties/pad/table.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/pad/table/pad_table_row
language: rust
---

# pad_table_row

v0.20 — Altium-style table data row. First cell is the row label

## Signature

```rust
pub(super) fn pad_table_row(
    label: &'a str,
    cells: Vec<iced::Element<'a, PanelMsg>>,
    portions: &[u16],
    primary: Color,
    _border_c: Color,
) -> iced::Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Docstring

v0.20 — Altium-style table data row. First cell is the row label
(e.g. "All Layers", "Pad Hole", "Top Paste"); remaining cells are
caller-provided Elements. Width portions match the header row.

## Source
Lines 59–81 in `crates/oxide-app/src/panels/footprint_editor_properties/pad/table.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [table](/crates/oxide-app/src/panels/footprint_editor_properties/pad/table.md) |
| called_by | [render_pad_form_pad_stack](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/stack/render_pad_form_pad_stack.md) |
| called_by | [pad_copper_row](/crates/oxide-app/src/panels/footprint_editor_properties/pad/table/pad_copper_row.md) |
