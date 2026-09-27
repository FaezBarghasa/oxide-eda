---
okf_version: "0.2"
type: Function
title: selected_is_deletable
description: "Whether [`delete_selected`] would actually remove anything for this"
resource: crates/oxide-app/src/library/editor/symbol/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/mod/selected_is_deletable
language: rust
---

# selected_is_deletable

Whether [`delete_selected`] would actually remove anything for this

## Signature

```rust
pub fn selected_is_deletable(selected: &Option<SymbolSelection>) -> bool
```

## Visibility

- `pub`

## Docstring

Whether [`delete_selected`] would actually remove anything for this
selection. `None`, `All`, and `Field` are no-ops there (they return
`None` without mutating the symbol), so callers gate on this to skip
a wasted undo snapshot — mirroring how [`selection_is_join_eligible`]
lets `apply_symbol_join` validate before it takes one — and to grey
out the context-menu Delete row for a selection it cannot act on.

## Source
Lines 520–527 in `crates/oxide-app/src/library/editor/symbol/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/symbol/state/mod.md) |
| called_by | [apply_symbol_transform](/crates/oxide-app/src/library/editor/symbol/updates/transform/apply_symbol_transform.md) |
