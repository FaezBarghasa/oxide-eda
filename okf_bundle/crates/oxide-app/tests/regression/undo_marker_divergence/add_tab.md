---
okf_version: "0.2"
type: Function
title: add_tab
description: "Insert an engine for `path` and give it a matching `TabInfo`, which"
resource: crates/oxide-app/tests/regression/undo_marker_divergence.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/undo_marker_divergence/add_tab
language: rust
---

# add_tab

Insert an engine for `path` and give it a matching `TabInfo`, which

## Signature

```rust
fn add_tab(app: &mut Oxide, path: &Path, sheet: SchematicSheet)
```

## Docstring

Insert an engine for `path` and give it a matching `TabInfo`, which
`finish_schematic_mutation` requires or it silently no-ops.

## Source
Lines 104–117 in `crates/oxide-app/tests/regression/undo_marker_divergence.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [undo_marker_divergence](/crates/oxide-app/tests/regression/undo_marker_divergence.md) |
| called_by | [a_pasted_batch_is_one_undo_step](/crates/oxide-app/tests/regression/undo_marker_divergence/a_pasted_batch_is_one_undo_step.md) |
| called_by | [an_undo_in_one_tab_spends_exactly_one_of_that_tabs_edits](/crates/oxide-app/tests/regression/undo_marker_divergence/an_undo_in_one_tab_spends_exactly_one_of_that_tabs_edits.md) |
| called_by | [fixture_two_symbols](/crates/oxide-app/tests/regression/undo_marker_divergence/fixture_two_symbols.md) |
