---
okf_version: "0.2"
type: Function
title: selection_has_enough_join_sources
description: "Whether `indices` names enough sources to plausibly close into a"
resource: crates/oxide-app/src/library/editor/symbol/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/mod/selection_has_enough_join_sources
language: rust
---

# selection_has_enough_join_sources

Whether `indices` names enough sources to plausibly close into a

## Signature

```rust
pub fn selection_has_enough_join_sources(sym: &Symbol, indices: &[usize]) -> bool
```

## Visibility

- `pub`

## Docstring

Whether `indices` names enough sources to plausibly close into a
ring: at least 2 (a lone pair might still fail to chain, but
that's a `chain_into_closed_contour` topology diagnosis, not
something worth pre-empting here), or exactly 1 *Arc* — a
sufficiently large sweep can legitimately self-close via its own
tiny chord gap (see `chain.rs`'s `near_full_sweep_arc_*` case). A
single `Line` can never close on its own; surfacing that as a
chain `OpenChain`/degenerate error would be misleading, so it's
disqualified outright instead.

## Source
Lines 482–491 in `crates/oxide-app/src/library/editor/symbol/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/symbol/state/mod.md) |
| called_by | [selection_is_join_eligible](/crates/oxide-app/src/library/editor/symbol/state/mod/selection_is_join_eligible.md) |
| called_by | [apply_symbol_join](/crates/oxide-app/src/library/editor/symbol/updates/join/apply_symbol_join.md) |
