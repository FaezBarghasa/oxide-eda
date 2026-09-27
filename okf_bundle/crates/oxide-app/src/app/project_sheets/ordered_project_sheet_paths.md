---
okf_version: "0.2"
type: Function
title: ordered_project_sheet_paths
description: The sheet-walk order every whole-project Annotate operation must agree
resource: crates/oxide-app/src/app/project_sheets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/project_sheets/ordered_project_sheet_paths
language: rust
---

# ordered_project_sheet_paths

The sheet-walk order every whole-project Annotate operation must agree

## Signature

```rust
pub(crate) fn ordered_project_sheet_paths(
    project_set: &ProjectSheetSet,
    active_path: Option<&Path>,
) -> Vec<PathBuf>
```

## Visibility

- `pub(crate)`

## Docstring

The sheet-walk order every whole-project Annotate operation must agree
on: `project_set.sheets` in sorted-path order, with `active_path`
appended at the end if it has a path the set doesn't already cover (an
active document the assembler can't place, e.g. one that hasn't been
saved anywhere the project reaches).

One function for the action (`handle_annotate`) and the preview list it
must match. #406 made them agree on *which* sheets to cover; they still
disagreed on the *order* to walk them in, which is what decides which
designator number each `?` symbol gets — a preview showing `R1` on sheet
A while the action hands sheet A `R2` (#435).

## Source
Lines 98–110 in `crates/oxide-app/src/app/project_sheets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_sheets](/crates/oxide-app/src/app/project_sheets.md) |
| called_by | [handle_annotate](/crates/oxide-app/src/app/handlers/erc/annotate/handle_annotate.md) |
| called_by | [preview_sheets](/crates/oxide-app/src/app/view/dialogs/annotate_preview/preview_sheets.md) |
