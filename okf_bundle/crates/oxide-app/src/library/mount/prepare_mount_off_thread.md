---
okf_version: "0.2"
type: Function
title: prepare_mount_off_thread
description: "Prepare a mount off the UI thread, wrapped for `Task::perform`."
resource: crates/oxide-app/src/library/mount.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/mount/prepare_mount_off_thread
language: rust
---

# prepare_mount_off_thread

Prepare a mount off the UI thread, wrapped for `Task::perform`.

## Signature

```rust
pub fn prepare_mount_off_thread(path: PathBuf) -> PreparedMountCell
```

## Visibility

- `pub`

## Docstring

Prepare a mount off the UI thread, wrapped for `Task::perform`.

Split out so both call sites — the browser open and the project
auto-mount fan-out — share one mechanism instead of growing two.

## Source
Lines 271–276 in `crates/oxide-app/src/library/mount.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mount](/crates/oxide-app/src/library/mount.md) |
| calls | [prepare_mount](/crates/oxide-app/src/library/mount/prepare_mount.md) |
| called_by | [handle_open_library_browser](/crates/oxide-app/src/app/dispatch/library/browser/mod/handle_open_library_browser.md) |
| called_by | [load_or_activate_project](/crates/oxide-app/src/app/handlers/document_files/open/load_or_activate_project.md) |
