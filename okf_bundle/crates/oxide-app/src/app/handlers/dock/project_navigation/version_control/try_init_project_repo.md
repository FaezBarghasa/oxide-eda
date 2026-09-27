---
okf_version: "0.2"
type: Function
title: try_init_project_repo
description: "Thin wrapper around `oxide_library::enable_project_version_control`"
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/version_control.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/version_control/try_init_project_repo
language: rust
---

# try_init_project_repo

Thin wrapper around `oxide_library::enable_project_version_control`

## Signature

```rust
fn try_init_project_repo(
    project_dir: &std::path::Path,
    use_lfs: bool,
    gitignore: Option<&str>,
) -> Result<(), oxide_library::LibraryError>
```

## Docstring

Thin wrapper around `oxide_library::enable_project_version_control`
— kept here so the dispatch handler can stay synchronous and
surface the `LibraryError` as a user-facing string. `gitignore`
is the body of the `.gitignore` to write before init (one line
per pattern, trailing newline); `None` skips the write entirely
so a fully-tracked initial commit stays bit-identical.

## Source
Lines 222–228 in `crates/oxide-app/src/app/handlers/dock/project_navigation/version_control.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [version_control](/crates/oxide-app/src/app/handlers/dock/project_navigation/version_control.md) |
| calls | [enable_project_version_control](/crates/oxide-library/src/lib/enable_project_version_control.md) |
| called_by | [handle_enable_version_control_confirm](/crates/oxide-app/src/app/handlers/dock/project_navigation/version_control/handle_enable_version_control_confirm.md) |
