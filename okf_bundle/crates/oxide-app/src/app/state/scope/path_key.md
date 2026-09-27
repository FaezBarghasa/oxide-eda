---
okf_version: "0.2"
type: Function
title: path_key
description: Comparison key for a filesystem path.
resource: crates/oxide-app/src/app/state/scope.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:50:25Z"
concept_id: crates/oxide-app/src/app/state/scope/path_key
language: rust
---

# path_key

Comparison key for a filesystem path.

## Signature

```rust
pub(crate) fn path_key(path: &Path) -> String
```

## Visibility

- `pub(crate)`

## Docstring

Comparison key for a filesystem path.

Windows filesystems are case-insensitive and accept either separator, but
Rust's `Path` comparison is neither — a `.snxprj` recording `Top.snxsch`
against a tab opened as `top.snxsch` (or a file dialog handing back a
differently-cased drive letter) would otherwise report the sheet as loose
and silently degrade a project export to a single page. Unix paths are
case-sensitive, so there the key is the path verbatim.

## Source
Lines 32–39 in `crates/oxide-app/src/app/state/scope.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scope](/crates/oxide-app/src/app/state/scope.md) |
| called_by | [assemble_project_sheets](/crates/oxide-app/src/app/project_sheets/assemble_project_sheets.md) |
| called_by | [sheet_key](/crates/oxide-app/src/app/project_sheets/sheet_key.md) |
| called_by | [walk](/crates/oxide-app/src/app/project_sheets/walk.md) |
| called_by | [parent_of](/crates/oxide-app/src/app/state/scope/parent_of.md) |
| called_by | [project_listing_sheet](/crates/oxide-app/src/app/state/scope/project_listing_sheet.md) |
| called_by | [project_owning_sheet](/crates/oxide-app/src/app/state/scope/project_owning_sheet.md) |
