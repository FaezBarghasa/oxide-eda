---
okf_version: "0.2"
type: Function
title: activate
description: "Make the tab whose document lives at `path` the active one — the same"
resource: crates/oxide-app/tests/regression/undo_marker_divergence.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/undo_marker_divergence/activate
language: rust
---

# activate

Make the tab whose document lives at `path` the active one — the same

## Signature

```rust
fn activate(app: &mut Oxide, path: &Path)
```

## Docstring

Make the tab whose document lives at `path` the active one — the same
two fields the tab-switch handlers set.

## Source
Lines 121–130 in `crates/oxide-app/tests/regression/undo_marker_divergence.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [undo_marker_divergence](/crates/oxide-app/tests/regression/undo_marker_divergence.md) |
| called_by | [a_pasted_batch_is_one_undo_step](/crates/oxide-app/tests/regression/undo_marker_divergence/a_pasted_batch_is_one_undo_step.md) |
| called_by | [an_undo_in_one_tab_spends_exactly_one_of_that_tabs_edits](/crates/oxide-app/tests/regression/undo_marker_divergence/an_undo_in_one_tab_spends_exactly_one_of_that_tabs_edits.md) |
| called_by | [fixture_two_symbols](/crates/oxide-app/tests/regression/undo_marker_divergence/fixture_two_symbols.md) |
