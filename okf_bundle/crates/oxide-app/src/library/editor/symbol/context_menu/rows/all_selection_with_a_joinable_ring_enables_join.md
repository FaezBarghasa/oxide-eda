---
okf_version: "0.2"
type: Function
title: all_selection_with_a_joinable_ring_enables_join
description: "`SymbolSelection::All` resolves to every visible graphic, so a"
resource: crates/oxide-app/src/library/editor/symbol/context_menu/rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/context_menu/rows/all_selection_with_a_joinable_ring_enables_join
language: rust
---

# all_selection_with_a_joinable_ring_enables_join

`SymbolSelection::All` resolves to every visible graphic, so a

## Signature

```rust
fn all_selection_with_a_joinable_ring_enables_join()
```

## Decorators

- `test`

## Docstring

`SymbolSelection::All` resolves to every visible graphic, so a
4-line ring selected via All enables Join into Polygon exactly
like the equivalent `Multiple` selection would.
[test]

## Source
Lines 294–313 in `crates/oxide-app/src/library/editor/symbol/context_menu/rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rows](/crates/oxide-app/src/library/editor/symbol/context_menu/rows.md) |
| calls | [build_symbol_context_menu_rows](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/build_symbol_context_menu_rows.md) |
