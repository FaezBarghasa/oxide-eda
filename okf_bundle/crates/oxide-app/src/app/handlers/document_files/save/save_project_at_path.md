---
okf_version: "0.2"
type: Function
title: save_project_at_path
description: "Save the loaded project whose `.snxprj` sits at `path`. Thin"
resource: crates/oxide-app/src/app/handlers/document_files/save.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/document_files/save/save_project_at_path
language: rust
---

# save_project_at_path

Save the loaded project whose `.snxprj` sits at `path`. Thin

## Signature

```rust
impl Oxide { pub(crate) fn save_project_at_path(&mut self, path: &std::path::Path) -> Result<()> }
```

## Visibility

- `pub(crate)`

## Docstring

Save the loaded project whose `.snxprj` sits at `path`. Thin
path-keyed wrapper over [`persist_project_by_id`] used by the
Save-All-on-close/exit dispatcher, which iterates dirty paths and
cannot assume the dirty `.snxprj` belongs to the *active*
project. Returns an error when no loaded project owns `path`, or
when the underlying `.snxprj` write fails.

## Source
Lines 142–151 in `crates/oxide-app/src/app/handlers/document_files/save.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [save](/crates/oxide-app/src/app/handlers/document_files/save.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
