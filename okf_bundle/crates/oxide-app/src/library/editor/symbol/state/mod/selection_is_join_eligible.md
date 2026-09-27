---
okf_version: "0.2"
type: Function
title: selection_is_join_eligible
description: "Whether the current selection is eligible for \"Join into Polygon\":"
resource: crates/oxide-app/src/library/editor/symbol/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/mod/selection_is_join_eligible
language: rust
---

# selection_is_join_eligible

Whether the current selection is eligible for "Join into Polygon":

## Signature

```rust
pub fn selection_is_join_eligible(
    sym: &Symbol,
    active_part: u8,
    selected: &Option<SymbolSelection>,
) -> bool
```

## Visibility

- `pub`

## Docstring

Whether the current selection is eligible for "Join into Polygon":
at least one graphic named, every named graphic is a `Line` or an
`Arc` (a Rectangle/Circle/Text/Polygon anywhere in the selection
disqualifies the whole op), the selection names enough sources to
plausibly close (see [`selection_has_enough_join_sources`]), and
every named graphic shares the same `part_number` (see
[`common_graphic_part_number`] — a selection mixing shared and
unit-specific sources disqualifies the whole op too, surfaced by
the caller as a distinct status message rather than a silent
no-op).

## Source
Lines 503–512 in `crates/oxide-app/src/library/editor/symbol/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/symbol/state/mod.md) |
| calls | [join_source_indices](/crates/oxide-app/src/library/editor/symbol/state/mod/join_source_indices.md) |
| calls | [selection_kinds_are_line_or_arc](/crates/oxide-app/src/library/editor/symbol/state/mod/selection_kinds_are_line_or_arc.md) |
| calls | [selection_has_enough_join_sources](/crates/oxide-app/src/library/editor/symbol/state/mod/selection_has_enough_join_sources.md) |
| calls | [common_graphic_part_number](/crates/oxide-app/src/library/editor/symbol/state/mod/common_graphic_part_number.md) |
| called_by | [apply_symbol_join](/crates/oxide-app/src/library/editor/symbol/updates/join/apply_symbol_join.md) |
