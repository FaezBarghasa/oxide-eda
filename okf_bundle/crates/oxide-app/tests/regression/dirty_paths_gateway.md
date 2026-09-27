---
okf_version: "0.2"
type: Module
title: dirty_paths_gateway
description: "#585 — every engine edit must reach `dirty_paths`."
resource: crates/oxide-app/tests/regression/dirty_paths_gateway.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/dirty_paths_gateway
language: rust
---

# dirty_paths_gateway

#585 — every engine edit must reach `dirty_paths`.

## Docstring

#585 — every engine edit must reach `dirty_paths`.

`DocumentState.dirty_paths` is the single source of truth for "this
file has unsaved edits". It is written only through
`with_active_schematic_session_mut`, which only
`finish_schematic_mutation` calls — so an edit that runs the engine
directly applies and repaints, and since #584 undoes correctly, but
leaves the app believing the document is clean.

The consequence a user meets is in
`quitting_after_a_move_selection_warns_instead_of_discarding_it`:
`handle_app_quit_requested` short-circuits on
`dirty_paths.is_empty()`, so the unsaved-changes modal never opens
and the edit goes out with the process.

Sites covered here: `erc/modals.rs` `handle_move_selection_apply` and
`handle_parameter_manager_edit`, and `erc/annotate.rs` `handle_annotate`
and `handle_reset_duplicate_designators` on the active engine.

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-app/tests/regression/dirty_paths_gateway/symbol.md) |
| related | [sheet_with](/crates/oxide-app/tests/regression/dirty_paths_gateway/sheet_with.md) |
| related | [app_with](/crates/oxide-app/tests/regression/dirty_paths_gateway/app_with.md) |
| related | [assert_dirty](/crates/oxide-app/tests/regression/dirty_paths_gateway/assert_dirty.md) |
| related | [move_selection](/crates/oxide-app/tests/regression/dirty_paths_gateway/move_selection.md) |
| related | [a_move_selection_marks_the_document_dirty](/crates/oxide-app/tests/regression/dirty_paths_gateway/a_move_selection_marks_the_document_dirty.md) |
| related | [quitting_after_a_move_selection_warns_instead_of_discarding_it](/crates/oxide-app/tests/regression/dirty_paths_gateway/quitting_after_a_move_selection_warns_instead_of_discarding_it.md) |
| related | [a_parameter_manager_edit_marks_the_document_dirty](/crates/oxide-app/tests/regression/dirty_paths_gateway/a_parameter_manager_edit_marks_the_document_dirty.md) |
| related | [annotate_marks_the_active_sheet_dirty](/crates/oxide-app/tests/regression/dirty_paths_gateway/annotate_marks_the_active_sheet_dirty.md) |
| related | [reset_duplicate_designators_marks_the_active_sheet_dirty](/crates/oxide-app/tests/regression/dirty_paths_gateway/reset_duplicate_designators_marks_the_active_sheet_dirty.md) |
