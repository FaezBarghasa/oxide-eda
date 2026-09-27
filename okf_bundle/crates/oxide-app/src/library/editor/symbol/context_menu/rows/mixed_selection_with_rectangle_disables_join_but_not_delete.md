---
okf_version: "0.2"
type: Function
title: mixed_selection_with_rectangle_disables_join_but_not_delete
description: A mixed selection containing a Rectangle disables Join into
resource: crates/oxide-app/src/library/editor/symbol/context_menu/rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/context_menu/rows/mixed_selection_with_rectangle_disables_join_but_not_delete
language: rust
---

# mixed_selection_with_rectangle_disables_join_but_not_delete

A mixed selection containing a Rectangle disables Join into

## Signature

```rust
fn mixed_selection_with_rectangle_disables_join_but_not_delete()
```

## Decorators

- `test`

## Docstring

A mixed selection containing a Rectangle disables Join into
Polygon, but Delete stays enabled (a non-empty selection is
still deletable even when it can't be joined).
[test]

## Source
Lines 319–347 in `crates/oxide-app/src/library/editor/symbol/context_menu/rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rows](/crates/oxide-app/src/library/editor/symbol/context_menu/rows.md) |
| calls | [build_symbol_context_menu_rows](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/build_symbol_context_menu_rows.md) |
