---
okf_version: "0.2"
type: Function
title: all_selection_disables_delete
description: "`SymbolSelection::All` is a no-op for `delete_selected`, so the"
resource: crates/oxide-app/src/library/editor/symbol/context_menu/rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/context_menu/rows/all_selection_disables_delete
language: rust
---

# all_selection_disables_delete

`SymbolSelection::All` is a no-op for `delete_selected`, so the

## Signature

```rust
fn all_selection_disables_delete()
```

## Decorators

- `test`

## Docstring

`SymbolSelection::All` is a no-op for `delete_selected`, so the
Delete row must be disabled — otherwise clicking it pushes a
wasted undo snapshot that can evict real history. Regression
guard for the delete-on-All undo-pollution fix.
[test]

## Source
Lines 203–212 in `crates/oxide-app/src/library/editor/symbol/context_menu/rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rows](/crates/oxide-app/src/library/editor/symbol/context_menu/rows.md) |
| calls | [build_symbol_context_menu_rows](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/build_symbol_context_menu_rows.md) |
