---
okf_version: "0.2"
type: Function
title: project_listing_sheet
description: "Project whose persisted `sheets` list names `path`."
resource: crates/oxide-app/src/app/state/scope.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:50:25Z"
concept_id: crates/oxide-app/src/app/state/scope/project_listing_sheet
language: rust
---

# project_listing_sheet

Project whose persisted `sheets` list names `path`.

## Signature

```rust
fn project_listing_sheet(
    projects: &'a [LoadedProject],
    path: &Path,
) -> Option<&'a LoadedProject>
```

## Type Parameters

- `'a`

## Docstring

Project whose persisted `sheets` list names `path`.

Stricter than `DocumentState::project_for_path`, which matches on the
parent directory alone: a schematic sitting inside a project's folder but
never added to it belongs to no project, and must not drag that project's
other sheets into an export.

## Source
Lines 47–59 in `crates/oxide-app/src/app/state/scope.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scope](/crates/oxide-app/src/app/state/scope.md) |
| calls | [path_key](/crates/oxide-app/src/app/state/scope/path_key.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [project_owning_sheet](/crates/oxide-app/src/app/state/scope/project_owning_sheet.md) |
