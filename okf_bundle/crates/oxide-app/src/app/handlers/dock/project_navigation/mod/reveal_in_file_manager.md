---
okf_version: "0.2"
type: Function
title: reveal_in_file_manager
description: "Open the OS file manager at `path`, selecting the file when the"
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/mod/reveal_in_file_manager
language: rust
---

# reveal_in_file_manager

Open the OS file manager at `path`, selecting the file when the

## Signature

```rust
fn reveal_in_file_manager(path: &std::path::Path) -> anyhow::Result<()>
```

## Docstring

Open the OS file manager at `path`, selecting the file when the
platform supports it (Windows `explorer /select,` and macOS `open
-R`). Linux/Unix fallback uses `xdg-open` on the parent directory
since most file managers don't accept a select-file argument.

## Source
Lines 243–294 in `crates/oxide-app/src/app/handlers/dock/project_navigation/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_navigation](/crates/oxide-app/src/app/handlers/dock/project_navigation/mod.md) |
| called_by | [handle_project_tree_action](/crates/oxide-app/src/app/handlers/dock/project_navigation/mod/handle_project_tree_action.md) |
