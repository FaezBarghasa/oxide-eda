---
okf_version: "0.2"
type: Function
title: tree_path_to_file_path
description: Resolve a project-tree path (indices) to the file path on disk
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/open_document.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/open_document/tree_path_to_file_path_1
language: rust
---

# tree_path_to_file_path

Resolve a project-tree path (indices) to the file path on disk

## Signature

```rust
pub(super) fn tree_path_to_file_path(&self, tree_path: &[usize]) -> Option<std::path::PathBuf>
```

## Visibility

- `pub(super)`

## Docstring

Resolve a project-tree path (indices) to the file path on disk
for the leaf node at that position. Multi-root aware: the first
index picks which project's directory to resolve against, so a
leaf under project B isn't accidentally resolved against project
A's parent directory.

F22 follow-up: `.snxlib` library leaves can live outside the
project directory (`LibraryEntryKind::Shared`). Resolve those
through `ProjectData::resolve_library_path` instead of joining
the leaf label against the project dir; otherwise the assembled
path doesn't exist and downstream remove / open paths bail
silently.

## Source
Lines 24–67 in `crates/oxide-app/src/app/handlers/dock/project_navigation/open_document.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [open_document](/crates/oxide-app/src/app/handlers/dock/project_navigation/open_document.md) |
| calls | [get_node](/crates/oxide-widgets/src/tree_view/get_node.md) |
| calls | [canonical_tree_label](/crates/oxide-app/src/app/handlers/dock/project_navigation/open_document/canonical_tree_label.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
