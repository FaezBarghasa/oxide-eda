---
okf_version: "0.2"
type: Function
title: handle_bom_preview_export
description: User clicked Export inside the BOM preview modal — stash the
resource: crates/oxide-app/src/app/handlers/menu/export/bom.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/bom/handle_bom_preview_export_1
language: rust
---

# handle_bom_preview_export

User clicked Export inside the BOM preview modal — stash the

## Signature

```rust
pub(crate) fn handle_bom_preview_export(&mut self) -> Option<Task<Message>>
```

## Visibility

- `pub(crate)`

## Docstring

User clicked Export inside the BOM preview modal — stash the
live options on the document, kick off the file dialog, and
finish in `handle_export_bom_finished`. Mirrors the
PrintPreview → Export PDF pattern; without `pending_bom_options`
the finish handler would fall back to defaults and the user's
column / grouping / variant picks would silently disappear
between modal-close and file-write.

## Source
Lines 201–244 in `crates/oxide-app/src/app/handlers/menu/export/bom.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bom](/crates/oxide-app/src/app/handlers/menu/export/bom.md) |
