---
okf_version: "0.2"
type: Function
title: pad_table_check_cell
description: v0.20 — checkbox cell for table data rows.
resource: crates/oxide-app/src/panels/footprint_editor_properties/pad/table.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/pad/table/pad_table_check_cell
language: rust
---

# pad_table_check_cell

v0.20 — checkbox cell for table data rows.

## Signature

```rust
pub(super) fn pad_table_check_cell(
    on: bool,
    on_toggle: impl Fn(bool) -> PanelMsg + 'a,
) -> iced::Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Docstring

v0.20 — checkbox cell for table data rows.

## Source
Lines 129–141 in `crates/oxide-app/src/panels/footprint_editor_properties/pad/table.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [table](/crates/oxide-app/src/panels/footprint_editor_properties/pad/table.md) |
