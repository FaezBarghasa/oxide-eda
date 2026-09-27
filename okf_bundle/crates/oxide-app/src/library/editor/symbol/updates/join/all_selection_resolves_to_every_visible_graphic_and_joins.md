---
okf_version: "0.2"
type: Function
title: all_selection_resolves_to_every_visible_graphic_and_joins
description: "`SymbolSelection::All` (box-select-everything / Ctrl+A / the"
resource: crates/oxide-app/src/library/editor/symbol/updates/join.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/join/all_selection_resolves_to_every_visible_graphic_and_joins
language: rust
---

# all_selection_resolves_to_every_visible_graphic_and_joins

`SymbolSelection::All` (box-select-everything / Ctrl+A / the

## Signature

```rust
fn all_selection_resolves_to_every_visible_graphic_and_joins()
```

## Decorators

- `test`

## Docstring

`SymbolSelection::All` (box-select-everything / Ctrl+A / the
Select All menu row) resolves to every graphic visible on the
active part, so joining a perfectly-selected ring via `All`
succeeds exactly like an explicit `Multiple` selection of the
same 4 graphics would.
[test]

## Source
Lines 514–529 in `crates/oxide-app/src/library/editor/symbol/updates/join.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [join](/crates/oxide-app/src/library/editor/symbol/updates/join.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/join/new_editor.md) |
| calls | [push_line](/crates/oxide-app/src/library/editor/symbol/updates/join/push_line.md) |
| calls | [apply_symbol_join](/crates/oxide-app/src/library/editor/symbol/updates/join/apply_symbol_join.md) |
