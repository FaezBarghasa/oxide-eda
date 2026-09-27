---
okf_version: "0.2"
type: Function
title: fixture_schematic_with_symbol_and_child_sheet
description: "A schematic engine with one plain `Symbol` (cuttable) and one"
resource: crates/oxide-app/tests/regression/project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:51:39Z"
concept_id: crates/oxide-app/tests/regression/project/fixture_schematic_with_symbol_and_child_sheet
language: rust
---

# fixture_schematic_with_symbol_and_child_sheet

A schematic engine with one plain `Symbol` (cuttable) and one

## Signature

```rust
fn fixture_schematic_with_symbol_and_child_sheet() -> (oxide_app::app::Oxide, uuid::Uuid, uuid::Uuid)
```

## Docstring

A schematic engine with one plain `Symbol` (cuttable) and one
`ChildSheet` (not cuttable — Copy drops it) already loaded and
made active, so `EditMsg::Cut` routes to
`handle_selection_cut_requested` against real engine state.
Returns the app plus both element UUIDs.

## Source
Lines 1022–1115 in `crates/oxide-app/tests/regression/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-app/tests/regression/project.md) |
| called_by | [cut_leaves_non_cuttable_child_sheet_in_place_and_selected](/crates/oxide-app/tests/regression/project/cut_leaves_non_cuttable_child_sheet_in_place_and_selected.md) |
