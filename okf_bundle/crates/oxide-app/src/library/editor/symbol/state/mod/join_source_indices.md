---
okf_version: "0.2"
type: Function
title: join_source_indices
description: Graphic indices the current selection names individually. Shared
resource: crates/oxide-app/src/library/editor/symbol/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/mod/join_source_indices
language: rust
---

# join_source_indices

Graphic indices the current selection names individually. Shared

## Signature

```rust
pub fn join_source_indices(
    sym: &Symbol,
    active_part: u8,
    selected: &Option<SymbolSelection>,
) -> Vec<usize>
```

## Visibility

- `pub`

## Docstring

Graphic indices the current selection names individually. Shared
by the "Join into Polygon" op (`updates::join`) and its
context-menu enablement check (`context_menu`) so both agree on
exactly which selections name eligible sources.

`All` (box-select-everything / Ctrl+A / the Select All menu row)
resolves to every graphic index visible on `active_part` — the
same [`graphic_on_part`] filter hit-test and box-select already
use — rather than the empty `Vec` it used to fall through to,
which disabled Join on a perfect ring the user had just selected
in full. `Pin` / `Field` still resolve to empty: neither names a
graphic.

## Source
Lines 423–438 in `crates/oxide-app/src/library/editor/symbol/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/symbol/state/mod.md) |
| calls | [graphic_on_part](/crates/oxide-app/src/library/editor/symbol/state/mod/graphic_on_part.md) |
| called_by | [selection_is_join_eligible](/crates/oxide-app/src/library/editor/symbol/state/mod/selection_is_join_eligible.md) |
| called_by | [apply_symbol_join](/crates/oxide-app/src/library/editor/symbol/updates/join/apply_symbol_join.md) |
