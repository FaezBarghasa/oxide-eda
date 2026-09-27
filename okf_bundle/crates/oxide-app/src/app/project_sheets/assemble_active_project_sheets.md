---
okf_version: "0.2"
type: Function
title: assemble_active_project_sheets
description: "The declared page paths of the project owning the active document, plus"
resource: crates/oxide-app/src/app/project_sheets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/project_sheets/assemble_active_project_sheets
language: rust
---

# assemble_active_project_sheets

The declared page paths of the project owning the active document, plus

## Signature

```rust
pub(crate) fn assemble_active_project_sheets(
    document_state: &DocumentState,
) -> (Vec<PathBuf>, ProjectSheetSet)
```

## Visibility

- `pub(crate)`

## Docstring

The declared page paths of the project owning the active document, plus
[`assemble_project_sheets`] over them — rooted at that project's root sheet,
or at the active document itself when it belongs to no project.

The entry point every whole-project operation calls: the export scope, the
cached canvas/ERC netlist, the ERC run, annotate and the duplicate-designator
reset. Deriving the page list and the root here, once, is the point — each
of those used to do it slightly differently and they disagreed (#406).

## Source
Lines 62–85 in `crates/oxide-app/src/app/project_sheets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_sheets](/crates/oxide-app/src/app/project_sheets.md) |
| calls | [assemble_project_sheets](/crates/oxide-app/src/app/project_sheets/assemble_project_sheets.md) |
| called_by | [handle_annotate](/crates/oxide-app/src/app/handlers/erc/annotate/handle_annotate.md) |
| called_by | [handle_reset_duplicate_designators](/crates/oxide-app/src/app/handlers/erc/annotate/handle_reset_duplicate_designators.md) |
| called_by | [handle_run_erc](/crates/oxide-app/src/app/handlers/erc/erc_run/handle_run_erc.md) |
| called_by | [build_export_scope](/crates/oxide-app/src/app/handlers/menu/export/mod/build_export_scope.md) |
| called_by | [preview_sheets](/crates/oxide-app/src/app/view/dialogs/annotate_preview/preview_sheets.md) |
