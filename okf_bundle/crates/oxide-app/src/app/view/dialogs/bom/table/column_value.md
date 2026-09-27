---
okf_version: "0.2"
type: Function
title: column_value
description: Rendered text of one cell — also the sort key for the text columns.
resource: crates/oxide-app/src/app/view/dialogs/bom/table.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/bom/table/column_value
language: rust
---

# column_value

Rendered text of one cell — also the sort key for the text columns.

## Signature

```rust
fn column_value(c: &oxide_output::BomColumn, r: &oxide_output::BomRow) -> String
```

## Docstring

Rendered text of one cell — also the sort key for the text columns.

## Source
Lines 18–30 in `crates/oxide-app/src/app/view/dialogs/bom/table.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [table](/crates/oxide-app/src/app/view/dialogs/bom/table.md) |
| called_by | [bom_table](/crates/oxide-app/src/app/view/dialogs/bom/table/bom_table.md) |
| called_by | [sorted_row_order](/crates/oxide-app/src/app/view/dialogs/bom/table/sorted_row_order.md) |
