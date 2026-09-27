---
okf_version: "0.2"
type: Module
title: tests
description: "Hierarchical child-sheet path resolution (#339, #406)."
resource: crates/oxide-app/src/app/handlers/canvas/tests.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:50:47Z"
concept_id: crates/oxide-app/src/app/handlers/canvas/tests
language: rust
---

# tests

Hierarchical child-sheet path resolution (#339, #406).

## Docstring

Hierarchical child-sheet path resolution (#339, #406).

A `ChildSheet.filename` is relative to the sheet that references it, not to
the project root — the convention `state::scope::parent_of` and
`project_sheets::project_graph` already use. Resolving it against the
project directory instead opens the wrong file (or reports "not found") for
any sheet that does not sit directly in the project root.

## Relationships

| Type | Target |
|------|--------|
| related | [app_focused_on](/crates/oxide-app/src/app/handlers/canvas/tests/app_focused_on.md) |
| related | [child_of_a_subdirectory_sheet_resolves_beside_its_parent](/crates/oxide-app/src/app/handlers/canvas/tests/child_of_a_subdirectory_sheet_resolves_beside_its_parent.md) |
| related | [child_of_a_root_sheet_still_resolves_in_the_project_root](/crates/oxide-app/src/app/handlers/canvas/tests/child_of_a_root_sheet_still_resolves_in_the_project_root.md) |
| related | [a_relative_child_reference_keeps_its_own_subpath](/crates/oxide-app/src/app/handlers/canvas/tests/a_relative_child_reference_keeps_its_own_subpath.md) |
| related | [an_absolute_child_reference_outside_the_root_is_rejected](/crates/oxide-app/src/app/handlers/canvas/tests/an_absolute_child_reference_outside_the_root_is_rejected.md) |
| related | [a_traversal_child_reference_outside_the_root_is_rejected](/crates/oxide-app/src/app/handlers/canvas/tests/a_traversal_child_reference_outside_the_root_is_rejected.md) |
| related | [an_empty_child_reference_resolves_to_nothing](/crates/oxide-app/src/app/handlers/canvas/tests/an_empty_child_reference_resolves_to_nothing.md) |
