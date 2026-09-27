---
okf_version: "0.2"
type: Function
title: selection_kinds_are_line_or_arc
description: "Whether every graphic `indices` names is a `Line` or an `Arc` — the"
resource: crates/oxide-app/src/library/editor/symbol/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/mod/selection_kinds_are_line_or_arc
language: rust
---

# selection_kinds_are_line_or_arc

Whether every graphic `indices` names is a `Line` or an `Arc` — the

## Signature

```rust
pub fn selection_kinds_are_line_or_arc(sym: &Symbol, indices: &[usize]) -> bool
```

## Visibility

- `pub`

## Docstring

Whether every graphic `indices` names is a `Line` or an `Arc` — the
source-*kind* half of Join-into-Polygon eligibility. `false` for an
empty list or any stale index. Exposed separately from
[`selection_is_join_eligible`] so a caller that needs to explain
*why* a selection is ineligible (as opposed to just gating on it)
can tell a kind mismatch apart from the part-number mismatch
[`common_graphic_part_number`] checks.

## Source
Lines 447–455 in `crates/oxide-app/src/library/editor/symbol/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/symbol/state/mod.md) |
| called_by | [selection_is_join_eligible](/crates/oxide-app/src/library/editor/symbol/state/mod/selection_is_join_eligible.md) |
| called_by | [apply_symbol_join](/crates/oxide-app/src/library/editor/symbol/updates/join/apply_symbol_join.md) |
