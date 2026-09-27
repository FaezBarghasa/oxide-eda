---
okf_version: "0.2"
type: Function
title: handle_enable_version_control_confirm
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/version_control.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/version_control/handle_enable_version_control_confirm
language: rust
---

# handle_enable_version_control_confirm

## Signature

```rust
impl Oxide { pub(crate) fn handle_enable_version_control_confirm(&mut self) }
```

## Visibility

- `pub(crate)`

## Source
Lines 101–213 in `crates/oxide-app/src/app/handlers/dock/project_navigation/version_control.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [version_control](/crates/oxide-app/src/app/handlers/dock/project_navigation/version_control.md) |
| calls | [build_gitignore_body](/crates/oxide-app/src/app/handlers/dock/project_navigation/version_control/build_gitignore_body.md) |
| calls | [try_init_project_repo](/crates/oxide-app/src/app/handlers/dock/project_navigation/version_control/try_init_project_repo.md) |
| calls | [log_warning](/crates/oxide-app/src/diagnostics/log_warning.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [log_info](/crates/oxide-app/src/diagnostics/log_info.md) |
