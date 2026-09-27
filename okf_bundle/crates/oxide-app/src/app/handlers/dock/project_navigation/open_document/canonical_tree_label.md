---
okf_version: "0.2"
type: Function
title: canonical_tree_label
description: "F24 — strip the \"  (missing)\" suffix `build_project_tree` appends"
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/open_document.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/open_document/canonical_tree_label
language: rust
---

# canonical_tree_label

F24 — strip the "  (missing)" suffix `build_project_tree` appends

## Signature

```rust
fn canonical_tree_label(label: &str) -> &str
```

## Docstring

F24 — strip the "  (missing)" suffix `build_project_tree` appends
to leaves whose backing file is absent from disk, so downstream
filename matching against `entry.path.file_name()` still works.
Returns the original `&str` when no suffix is present (zero-copy).

## Source
Lines 219–221 in `crates/oxide-app/src/app/handlers/dock/project_navigation/open_document.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [open_document](/crates/oxide-app/src/app/handlers/dock/project_navigation/open_document.md) |
| called_by | [open_project_tree_document](/crates/oxide-app/src/app/handlers/dock/project_navigation/open_document/open_project_tree_document.md) |
| called_by | [tree_path_to_file_path](/crates/oxide-app/src/app/handlers/dock/project_navigation/open_document/tree_path_to_file_path.md) |
