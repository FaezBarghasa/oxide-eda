---
okf_version: "0.2"
type: Function
title: pad_table_input_cell
description: "v0.20 — text_input cell with the same chrome as `pad_input_row`'s"
resource: crates/oxide-app/src/panels/footprint_editor_properties/pad/table.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/pad/table/pad_table_input_cell
language: rust
---

# pad_table_input_cell

v0.20 — text_input cell with the same chrome as `pad_input_row`'s

## Signature

```rust
pub(super) fn pad_table_input_cell(
    value: String,
    placeholder: &'a str,
    on_input: impl Fn(String) -> PanelMsg + 'a,
    muted: Color,
    primary: Color,
    border_c: Color,
) -> iced::Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Docstring

v0.20 — text_input cell with the same chrome as `pad_input_row`'s
input but no leading label — meant for table data rows.

## Source
Lines 85–110 in `crates/oxide-app/src/panels/footprint_editor_properties/pad/table.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [table](/crates/oxide-app/src/panels/footprint_editor_properties/pad/table.md) |
