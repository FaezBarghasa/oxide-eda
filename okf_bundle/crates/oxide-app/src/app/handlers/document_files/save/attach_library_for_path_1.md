---
okf_version: "0.2"
type: Function
title: attach_library_for_path
description: "Walk `path`'s ancestors looking for a `.snxlib` directory and,"
resource: crates/oxide-app/src/app/handlers/document_files/save.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/document_files/save/attach_library_for_path_1
language: rust
---

# attach_library_for_path

Walk `path`'s ancestors looking for a `.snxlib` directory and,

## Signature

```rust
fn attach_library_for_path(&mut self, path: &std::path::Path)
```

## Docstring

Walk `path`'s ancestors looking for a `.snxlib` directory and,
if found, make sure the active project has a `LibraryEntry`
pointing at it. Mounts the library on the `LibrarySet` if it's
not mounted yet. Logs and skips when there's no `.snxlib`
ancestor (loose-file save outside the library system) or when
no project is active to attach to.

## Source
Lines 379–492 in `crates/oxide-app/src/app/handlers/document_files/save.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [save](/crates/oxide-app/src/app/handlers/document_files/save.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
