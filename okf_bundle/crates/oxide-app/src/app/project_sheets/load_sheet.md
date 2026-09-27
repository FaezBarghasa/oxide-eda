---
okf_version: "0.2"
type: Function
title: load_sheet
description: "Read one sheet: the live engine snapshot when the file is open as a tab (so"
resource: crates/oxide-app/src/app/project_sheets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/project_sheets/load_sheet
language: rust
---

# load_sheet

Read one sheet: the live engine snapshot when the file is open as a tab (so

## Signature

```rust
fn load_sheet(
    document_state: &DocumentState,
    path: &Path,
) -> Result<Option<SchematicSheet>, String>
```

## Docstring

Read one sheet: the live engine snapshot when the file is open as a tab (so
unsaved edits are in the answer), a disk parse otherwise.

`Ok(None)` means genuinely absent; `Err` means present but unusable.

## Source
Lines 129–145 in `crates/oxide-app/src/app/project_sheets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_sheets](/crates/oxide-app/src/app/project_sheets.md) |
| called_by | [walk](/crates/oxide-app/src/app/project_sheets/walk.md) |
