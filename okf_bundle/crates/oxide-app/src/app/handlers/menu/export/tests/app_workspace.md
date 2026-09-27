---
okf_version: "0.2"
type: Function
title: app_workspace
description: "A `Oxide` with one loaded, *active* project whose persisted sheet list is"
resource: crates/oxide-app/src/app/handlers/menu/export/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:50:59Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/tests/app_workspace
language: rust
---

# app_workspace

A `Oxide` with one loaded, *active* project whose persisted sheet list is

## Signature

```rust
pub(crate) fn app_workspace(dir: &str, listed: &[&str]) -> Oxide
```

## Visibility

- `pub(crate)`

## Docstring

A `Oxide` with one loaded, *active* project whose persisted sheet list is
`listed`. Sheets are not opened here — callers add the engines they need
with [`open`].

## Source
Lines 199–231 in `crates/oxide-app/src/app/handlers/menu/export/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/app/handlers/menu/export/tests.md) |
| called_by | [app_flat_project](/crates/oxide-app/src/app/handlers/menu/export/tests/app_flat_project.md) |
| called_by | [app_with_missing_child](/crates/oxide-app/src/app/handlers/menu/export/tests/app_with_missing_child.md) |
| called_by | [workspace](/crates/oxide-app/src/app/handlers/menu/export/tests/workspace.md) |
| called_by | [app_with_a_child_only_on_disk](/crates/oxide-app/src/app/mutation_gateway/app_with_a_child_only_on_disk.md) |
| called_by | [resetting_duplicate_designators_sees_a_child_that_is_only_on_disk](/crates/oxide-app/src/app/mutation_gateway/resetting_duplicate_designators_sees_a_child_that_is_only_on_disk.md) |
| called_by | [fixture](/crates/oxide-app/src/app/view/dialogs/annotate_preview/tests/fixture.md) |
