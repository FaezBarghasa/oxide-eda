---
okf_version: "0.2"
type: Function
title: pad_table_picklist_cell
description: v0.20 — pick_list cell for table data rows.
resource: crates/oxide-app/src/panels/footprint_editor_properties/pad/table.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/pad/table/pad_table_picklist_cell
language: rust
---

# pad_table_picklist_cell

v0.20 — pick_list cell for table data rows.

## Signature

```rust
pub(super) fn pad_table_picklist_cell(
    options: &'a [T],
    selected: T,
    on_change: impl Fn(T) -> PanelMsg + 'a + Clone,
) -> iced::Element<'a, PanelMsg>
```

## Type Parameters

- `'a`
- `T`

## Visibility

- `pub(super)`

## Docstring

v0.20 — pick_list cell for table data rows.

## Source
Lines 113–126 in `crates/oxide-app/src/panels/footprint_editor_properties/pad/table.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [table](/crates/oxide-app/src/panels/footprint_editor_properties/pad/table.md) |
