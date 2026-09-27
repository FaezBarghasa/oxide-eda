---
okf_version: "0.2"
type: Function
title: handle_app_quit_requested
description: "Entry point for every app-exit request — chrome ✕, File ▸ Exit,"
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/close_project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/close_project/handle_app_quit_requested
language: rust
---

# handle_app_quit_requested

Entry point for every app-exit request — chrome ✕, File ▸ Exit,

## Signature

```rust
impl Oxide { pub(crate) fn handle_app_quit_requested(&mut self) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Docstring

Entry point for every app-exit request — chrome ✕, File ▸ Exit,
and OS close (Alt+F4) all funnel here. If any document in the
workspace has unsaved edits (`dirty_paths` non-empty), opens the
app-quit confirmation modal instead of exiting; otherwise closes
the main window, which the daemon turns into process exit via
`SecondaryWindowClosed`.

## Source
Lines 287–302 in `crates/oxide-app/src/app/handlers/dock/project_navigation/close_project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [close_project](/crates/oxide-app/src/app/handlers/dock/project_navigation/close_project.md) |
