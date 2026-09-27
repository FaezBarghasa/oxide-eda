---
okf_version: "0.2"
type: Function
title: project_root_sheet_path
description: "Absolute path of a project's root schematic — its declared"
resource: crates/oxide-app/src/app/project_sheets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/project_sheets/project_root_sheet_path
language: rust
---

# project_root_sheet_path

Absolute path of a project's root schematic — its declared

## Signature

```rust
pub(crate) fn project_root_sheet_path(
    project: &crate::app::state::LoadedProject,
) -> Option<PathBuf>
```

## Visibility

- `pub(crate)`

## Docstring

Absolute path of a project's root schematic — its declared
`schematic_root`, falling back to the first entry in the sheet list.

## Source
Lines 114–123 in `crates/oxide-app/src/app/project_sheets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_sheets](/crates/oxide-app/src/app/project_sheets.md) |
