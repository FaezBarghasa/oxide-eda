---
okf_version: "0.2"
type: Function
title: build_gitignore_body
description: "Render the user's tick-list into a `.gitignore` body. Returns an"
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/version_control.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/version_control/build_gitignore_body
language: rust
---

# build_gitignore_body

Render the user's tick-list into a `.gitignore` body. Returns an

## Signature

```rust
pub(crate) fn build_gitignore_body(items: &[crate::app::TrackItem]) -> String
```

## Visibility

- `pub(crate)`

## Docstring

Render the user's tick-list into a `.gitignore` body. Returns an
empty string when every row is ticked (no exclusions needed) so
the caller can skip writing a no-op file. Directory rows get a
trailing slash so git matches the directory and its contents.

## Source
Lines 357–385 in `crates/oxide-app/src/app/handlers/dock/project_navigation/version_control.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [version_control](/crates/oxide-app/src/app/handlers/dock/project_navigation/version_control.md) |
| called_by | [handle_enable_version_control_confirm](/crates/oxide-app/src/app/handlers/dock/project_navigation/version_control/handle_enable_version_control_confirm.md) |
