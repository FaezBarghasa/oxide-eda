---
okf_version: "0.2"
type: Function
title: app_focused_on
description: "An app with one loaded project whose `.snxprj` is at `/w/a`, listing both"
resource: crates/oxide-app/src/app/handlers/canvas/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:50:47Z"
concept_id: crates/oxide-app/src/app/handlers/canvas/tests/app_focused_on
language: rust
---

# app_focused_on

An app with one loaded project whose `.snxprj` is at `/w/a`, listing both

## Signature

```rust
fn app_focused_on(focused: &str) -> Oxide
```

## Docstring

An app with one loaded project whose `.snxprj` is at `/w/a`, listing both
`top.snxsch` and `sub/mid.snxsch`, and one tab focused on `focused`.

Both sheets are *listed*, which matters: it is what makes
`active_document_project()` resolve to ProjectA, which is what made the old
project-relative base directory fire. A fixture whose focused sheet is
unowned falls through to the loose-document branch and cannot go red.

## Source
Lines 22–64 in `crates/oxide-app/src/app/handlers/canvas/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/app/handlers/canvas/tests.md) |
| called_by | [a_relative_child_reference_keeps_its_own_subpath](/crates/oxide-app/src/app/handlers/canvas/tests/a_relative_child_reference_keeps_its_own_subpath.md) |
| called_by | [a_traversal_child_reference_outside_the_root_is_rejected](/crates/oxide-app/src/app/handlers/canvas/tests/a_traversal_child_reference_outside_the_root_is_rejected.md) |
| called_by | [an_absolute_child_reference_outside_the_root_is_rejected](/crates/oxide-app/src/app/handlers/canvas/tests/an_absolute_child_reference_outside_the_root_is_rejected.md) |
| called_by | [an_empty_child_reference_resolves_to_nothing](/crates/oxide-app/src/app/handlers/canvas/tests/an_empty_child_reference_resolves_to_nothing.md) |
| called_by | [child_of_a_root_sheet_still_resolves_in_the_project_root](/crates/oxide-app/src/app/handlers/canvas/tests/child_of_a_root_sheet_still_resolves_in_the_project_root.md) |
| called_by | [child_of_a_subdirectory_sheet_resolves_beside_its_parent](/crates/oxide-app/src/app/handlers/canvas/tests/child_of_a_subdirectory_sheet_resolves_beside_its_parent.md) |
