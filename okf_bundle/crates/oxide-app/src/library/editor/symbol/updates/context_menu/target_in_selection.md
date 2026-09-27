---
okf_version: "0.2"
type: Function
title: target_in_selection
description: "Whether `target` (a right-clicked pin/graphic) is already part of"
resource: crates/oxide-app/src/library/editor/symbol/updates/context_menu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/context_menu/target_in_selection
language: rust
---

# target_in_selection

Whether `target` (a right-clicked pin/graphic) is already part of

## Signature

```rust
fn target_in_selection(
    selected: &Option<crate::library::editor::symbol::state::SymbolSelection>,
    target: SymbolContextTarget,
) -> bool
```

## Docstring

Whether `target` (a right-clicked pin/graphic) is already part of
`selected` — a lone `Pin`/`Graphic` match, a member of a
`Multiple`'s index lists, or `All` (which covers every pin and
graphic). `Empty` never matches (there's nothing to preserve a
selection against on bare canvas).

## Source
Lines 67–92 in `crates/oxide-app/src/library/editor/symbol/updates/context_menu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context_menu](/crates/oxide-app/src/library/editor/symbol/updates/context_menu.md) |
| called_by | [apply_symbol_context_menu](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/apply_symbol_context_menu.md) |
