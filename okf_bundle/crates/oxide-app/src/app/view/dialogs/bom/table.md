---
okf_version: "0.2"
type: Module
title: table
description: "BOM preview — the spreadsheet-style data grid (row-number gutter,"
resource: crates/oxide-app/src/app/view/dialogs/bom/table.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/bom/table
language: rust
---

# table

BOM preview — the spreadsheet-style data grid (row-number gutter,

## Docstring

BOM preview — the spreadsheet-style data grid (row-number gutter,
draggable/resizable/sortable column headers, and the scrollable body
rows). Extracted from `dialogs/bom.rs` (ADR-0001, issue #164) as pure
code motion — the child-push order inside every row/column is preserved
byte-for-byte, so the rendered table is pixel-identical.

## Relationships

| Type | Target |
|------|--------|
| related | [first_reference](/crates/oxide-app/src/app/view/dialogs/bom/table/first_reference.md) |
| related | [column_value](/crates/oxide-app/src/app/view/dialogs/bom/table/column_value.md) |
| related | [sorted_row_order](/crates/oxide-app/src/app/view/dialogs/bom/table/sorted_row_order.md) |
| related | [bom_table](/crates/oxide-app/src/app/view/dialogs/bom/table/bom_table.md) |
| related | [bom_table](/crates/oxide-app/src/app/view/dialogs/bom/table/bom_table.md) |
| related | [row](/crates/oxide-app/src/app/view/dialogs/bom/table/row.md) |
| related | [designator_sort_matches_the_export_order](/crates/oxide-app/src/app/view/dialogs/bom/table/designator_sort_matches_the_export_order.md) |
| related | [other_columns_keep_their_own_orders](/crates/oxide-app/src/app/view/dialogs/bom/table/other_columns_keep_their_own_orders.md) |
| related | [unsorted_and_out_of_range_keep_rollup_order](/crates/oxide-app/src/app/view/dialogs/bom/table/unsorted_and_out_of_range_keep_rollup_order.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
