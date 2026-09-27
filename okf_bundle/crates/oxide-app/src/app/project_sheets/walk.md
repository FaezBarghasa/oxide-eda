---
okf_version: "0.2"
type: Function
title: walk
description: "Breadth-first load of `seeds` and their `child_sheets` descendants into"
resource: crates/oxide-app/src/app/project_sheets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/project_sheets/walk
language: rust
---

# walk

Breadth-first load of `seeds` and their `child_sheets` descendants into

## Signature

```rust
fn walk(
    document_state: &DocumentState,
    seeds: &[PathBuf],
    visited: &mut HashSet<String>,
    set: &mut ProjectSheetSet,
) -> HashSet<String>
```

## Docstring

Breadth-first load of `seeds` and their `child_sheets` descendants into
`set`. Returns the `path_key`s actually loaded by this walk.

## Source
Lines 199–234 in `crates/oxide-app/src/app/project_sheets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_sheets](/crates/oxide-app/src/app/project_sheets.md) |
| calls | [path_key](/crates/oxide-app/src/app/state/scope/path_key.md) |
| calls | [load_sheet](/crates/oxide-app/src/app/project_sheets/load_sheet.md) |
| called_by | [assemble_project_sheets](/crates/oxide-app/src/app/project_sheets/assemble_project_sheets.md) |
