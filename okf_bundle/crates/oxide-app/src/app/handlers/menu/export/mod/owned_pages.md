---
okf_version: "0.2"
type: Function
title: owned_pages
description: "One page per project sheet entry, in list order. Sheets open as tabs carry"
resource: crates/oxide-app/src/app/handlers/menu/export/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/mod/owned_pages
language: rust
---

# owned_pages

One page per project sheet entry, in list order. Sheets open as tabs carry

## Signature

```rust
fn owned_pages(
    project: &crate::app::state::LoadedProject,
    pages: &[PathBuf],
    sheet_set: &crate::app::project_sheets::ProjectSheetSet,
) -> Vec<SheetSnapshot>
```

## Docstring

One page per project sheet entry, in list order. Sheets open as tabs carry
the live engine snapshot (so unsaved edits show in the preview); the rest
were read from disk by the assembler.

## Source
Lines 145–176 in `crates/oxide-app/src/app/handlers/menu/export/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [export](/crates/oxide-app/src/app/handlers/menu/export/mod.md) |
| called_by | [build_export_scope](/crates/oxide-app/src/app/handlers/menu/export/mod/build_export_scope.md) |
