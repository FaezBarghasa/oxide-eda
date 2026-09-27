---
okf_version: "0.2"
type: Module
title: library_undo
description: "`push_history` / `undo` / `redo` in isolation, no placement or geometry involved."
resource: crates/oxide-app/tests/regression/library_undo.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/tests/regression/library_undo
language: rust
---

# library_undo

`push_history` / `undo` / `redo` in isolation, no placement or geometry involved.

## Docstring

`push_history` / `undo` / `redo` in isolation, no placement or geometry involved.

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_editor_push_history_then_undo_restores_pads](/crates/oxide-app/tests/regression/library_undo/footprint_editor_push_history_then_undo_restores_pads.md) |
| related | [footprint_editor_undo_returns_false_on_empty_history](/crates/oxide-app/tests/regression/library_undo/footprint_editor_undo_returns_false_on_empty_history.md) |
| related | [footprint_editor_history_caps_at_depth_limit](/crates/oxide-app/tests/regression/library_undo/footprint_editor_history_caps_at_depth_limit.md) |
| related | [footprint_editor_new_mutation_clears_redo_stack](/crates/oxide-app/tests/regression/library_undo/footprint_editor_new_mutation_clears_redo_stack.md) |
