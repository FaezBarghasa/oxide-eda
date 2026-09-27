---
okf_version: "0.2"
type: Function
title: app_with
description: "A loaded, active schematic. The `TabInfo` matters:"
resource: crates/oxide-app/tests/regression/dirty_paths_gateway.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/dirty_paths_gateway/app_with
language: rust
---

# app_with

A loaded, active schematic. The `TabInfo` matters:

## Signature

```rust
fn app_with(symbols: Vec<Symbol>) -> (Oxide, PathBuf)
```

## Docstring

A loaded, active schematic. The `TabInfo` matters:
`finish_schematic_mutation` reaches `dirty_paths` through
`with_active_schematic_session_mut`, which needs a tab at
`active_tab` or it silently no-ops.

## Source
Lines 89–107 in `crates/oxide-app/tests/regression/dirty_paths_gateway.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dirty_paths_gateway](/crates/oxide-app/tests/regression/dirty_paths_gateway.md) |
| calls | [sheet_with](/crates/oxide-app/tests/regression/dirty_paths_gateway/sheet_with.md) |
| called_by | [a_move_selection_marks_the_document_dirty](/crates/oxide-app/tests/regression/dirty_paths_gateway/a_move_selection_marks_the_document_dirty.md) |
| called_by | [a_parameter_manager_edit_marks_the_document_dirty](/crates/oxide-app/tests/regression/dirty_paths_gateway/a_parameter_manager_edit_marks_the_document_dirty.md) |
| called_by | [annotate_marks_the_active_sheet_dirty](/crates/oxide-app/tests/regression/dirty_paths_gateway/annotate_marks_the_active_sheet_dirty.md) |
| called_by | [quitting_after_a_move_selection_warns_instead_of_discarding_it](/crates/oxide-app/tests/regression/dirty_paths_gateway/quitting_after_a_move_selection_warns_instead_of_discarding_it.md) |
| called_by | [reset_duplicate_designators_marks_the_active_sheet_dirty](/crates/oxide-app/tests/regression/dirty_paths_gateway/reset_duplicate_designators_marks_the_active_sheet_dirty.md) |
