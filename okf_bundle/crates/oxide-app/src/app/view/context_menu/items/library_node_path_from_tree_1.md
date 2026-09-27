---
okf_version: "0.2"
type: Function
title: library_node_path_from_tree
description: "Resolve the on-disk `.snxlib` path for a project-tree library"
resource: crates/oxide-app/src/app/view/context_menu/items.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/context_menu/items/library_node_path_from_tree_1
language: rust
---

# library_node_path_from_tree

Resolve the on-disk `.snxlib` path for a project-tree library

## Signature

```rust
pub(super) fn library_node_path_from_tree(
        &self,
        tree_path: &[usize],
    ) -> Option<std::path::PathBuf>
```

## Visibility

- `pub(super)`

## Docstring

Resolve the on-disk `.snxlib` path for a project-tree library
node click. The tree path under the `Libraries` group is
`[project_idx, libraries_branch_idx, library_idx]` — the
matching project's `LibraryNodeInfo[library_idx].root` carries
the absolute path. Returns `None` when the path doesn't sit at
a library leaf (defensive — `view_project_tree_context_menu`
already gates on the library role, but the icon-by-depth
detection there isn't strict enough to skip the lookup).

## Source
Lines 186–199 in `crates/oxide-app/src/app/view/context_menu/items.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [items](/crates/oxide-app/src/app/view/context_menu/items.md) |
