---
okf_version: "0.2"
type: Function
title: open_library_enable_version_control_dialog
description: "v0.11 library-node: open the same Enable Version Control modal"
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/version_control.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/version_control/open_library_enable_version_control_dialog
language: rust
---

# open_library_enable_version_control_dialog

v0.11 library-node: open the same Enable Version Control modal

## Signature

```rust
impl Oxide { pub(crate) fn open_library_enable_version_control_dialog(&mut self, tree_path: Vec<usize>) }
```

## Visibility

- `pub(crate)`

## Docstring

v0.11 library-node: open the same Enable Version Control modal
scoped to a single `.snxlib` directory rather than the whole
project tree. The library context-menu only surfaces this when
the library's `root_dir` has no `.git/` already.

## Source
Lines 49–99 in `crates/oxide-app/src/app/handlers/dock/project_navigation/version_control.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [version_control](/crates/oxide-app/src/app/handlers/dock/project_navigation/version_control.md) |
| calls | [collect_track_items_for_library](/crates/oxide-app/src/app/handlers/dock/project_navigation/version_control/collect_track_items_for_library.md) |
