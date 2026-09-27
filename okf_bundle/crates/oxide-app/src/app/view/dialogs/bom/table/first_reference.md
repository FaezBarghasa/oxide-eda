---
okf_version: "0.2"
type: Function
title: first_reference
description: "First designator of a BOM row — the key the export pipeline orders rows by,"
resource: crates/oxide-app/src/app/view/dialogs/bom/table.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/bom/table/first_reference
language: rust
---

# first_reference

First designator of a BOM row — the key the export pipeline orders rows by,

## Signature

```rust
fn first_reference(row: &oxide_output::BomRow) -> &str
```

## Docstring

First designator of a BOM row — the key the export pipeline orders rows by,
so the preview's Designator sort matches the exported file exactly.

## Source
Lines 13–15 in `crates/oxide-app/src/app/view/dialogs/bom/table.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [table](/crates/oxide-app/src/app/view/dialogs/bom/table.md) |
| called_by | [sorted_row_order](/crates/oxide-app/src/app/view/dialogs/bom/table/sorted_row_order.md) |
