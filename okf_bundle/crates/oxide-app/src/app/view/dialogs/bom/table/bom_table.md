---
okf_version: "0.2"
type: Function
title: bom_table
description: Build the scrollable BOM data grid (header strip + data rows) for the
resource: crates/oxide-app/src/app/view/dialogs/bom/table.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/bom/table/bom_table
language: rust
---

# bom_table

Build the scrollable BOM data grid (header strip + data rows) for the

## Signature

```rust
impl Oxide { pub(super) fn bom_table(&self) -> Element<'_, Message> }
```

## Visibility

- `pub(super)`

## Docstring

Build the scrollable BOM data grid (header strip + data rows) for the
active preview. Returns the `scrollable` body the modal drops into its
main row.

## Source
Lines 76–371 in `crates/oxide-app/src/app/view/dialogs/bom/table.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [table](/crates/oxide-app/src/app/view/dialogs/bom/table.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [toolbar_strip](/crates/oxide-app/src/styles/toolbar_strip.md) |
| calls | [sorted_row_order](/crates/oxide-app/src/app/view/dialogs/bom/table/sorted_row_order.md) |
| calls | [cell](/crates/oxide-app/tests/command_reference/cell.md) |
| calls | [column_value](/crates/oxide-app/src/app/view/dialogs/bom/table/column_value.md) |
