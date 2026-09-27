---
okf_version: "0.2"
type: Function
title: spawn_save_as_for_new_primitive
description: "Build the AsyncFileDialog Task for a primitive's first save."
resource: crates/oxide-app/src/app/handlers/document_files/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/document_files/mod/spawn_save_as_for_new_primitive
language: rust
---

# spawn_save_as_for_new_primitive

Build the AsyncFileDialog Task for a primitive's first save.

## Signature

```rust
pub(crate) fn spawn_save_as_for_new_primitive(suggested: PathBuf) -> iced::Task<Message>
```

## Visibility

- `pub(crate)`

## Docstring

Build the AsyncFileDialog Task for a primitive's first save.
The dialog defaults to the suggested path's parent + filename so
the common case is a single Enter key; the user can navigate to
a global library directory outside the project if they want a
shared symbol. Cancel = no save (editor stays dirty).

## Source
Lines 17–60 in `crates/oxide-app/src/app/handlers/document_files/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [document_files](/crates/oxide-app/src/app/handlers/document_files/mod.md) |
| called_by | [handle_primitive_editor_event](/crates/oxide-app/src/app/dispatch/library/editor/handle_primitive_editor_event.md) |
| called_by | [save_active_document](/crates/oxide-app/src/app/handlers/document_files/save/save_active_document.md) |
