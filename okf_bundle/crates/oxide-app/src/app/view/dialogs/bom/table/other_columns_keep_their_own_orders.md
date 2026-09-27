---
okf_version: "0.2"
type: Function
title: other_columns_keep_their_own_orders
description: Qty stays numeric and the text columns stay case-insensitive — the
resource: crates/oxide-app/src/app/view/dialogs/bom/table.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/bom/table/other_columns_keep_their_own_orders
language: rust
---

# other_columns_keep_their_own_orders

Qty stays numeric and the text columns stay case-insensitive — the

## Signature

```rust
fn other_columns_keep_their_own_orders()
```

## Decorators

- `test`

## Docstring

Qty stays numeric and the text columns stay case-insensitive — the
natural-order change must not have leaked into the other branches.
[test]

## Source
Lines 413–429 in `crates/oxide-app/src/app/view/dialogs/bom/table.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [table](/crates/oxide-app/src/app/view/dialogs/bom/table.md) |
