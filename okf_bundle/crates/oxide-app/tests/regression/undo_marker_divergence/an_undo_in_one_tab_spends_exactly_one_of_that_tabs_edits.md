---
okf_version: "0.2"
type: Function
title: an_undo_in_one_tab_spends_exactly_one_of_that_tabs_edits
description: "Undo is per document, and one press spends exactly one of the active"
resource: crates/oxide-app/tests/regression/undo_marker_divergence.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/undo_marker_divergence/an_undo_in_one_tab_spends_exactly_one_of_that_tabs_edits
language: rust
---

# an_undo_in_one_tab_spends_exactly_one_of_that_tabs_edits

Undo is per document, and one press spends exactly one of the active

## Signature

```rust
fn an_undo_in_one_tab_spends_exactly_one_of_that_tabs_edits()
```

## Decorators

- `test`

## Docstring

Undo is per document, and one press spends exactly one of the active
document's edits.

This is the cross-tab corruption in its sharpest form. The marker
stack was one *global* stack across every open document's per-path
engine and was never cleared on tab switch, so the count on top of it
could describe a different document than the one being undone. Three
separate edits in tab B, then a three-object paste in tab A, left the
top marker reading `steps: 3` — and a single Undo pressed back in tab
B ran `Engine::undo` three times on B, wiping all three of B's
unrelated edits in one keystroke.

The count matters: an Undo in a tab with *nothing* to undo was
already harmless, because the old `undone_steps != steps` guard
short-circuited. It is the mismatched count, not the mere sharing,
that destroyed work.
[test]

## Source
Lines 353–433 in `crates/oxide-app/tests/regression/undo_marker_divergence.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [undo_marker_divergence](/crates/oxide-app/tests/regression/undo_marker_divergence.md) |
| calls | [add_tab](/crates/oxide-app/tests/regression/undo_marker_divergence/add_tab.md) |
| calls | [sheet_with](/crates/oxide-app/tests/regression/undo_marker_divergence/sheet_with.md) |
| calls | [symbol](/crates/oxide-app/tests/regression/undo_marker_divergence/symbol.md) |
| calls | [activate](/crates/oxide-app/tests/regression/undo_marker_divergence/activate.md) |
