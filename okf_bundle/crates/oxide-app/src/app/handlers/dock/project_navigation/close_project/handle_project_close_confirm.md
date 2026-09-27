---
okf_version: "0.2"
type: Function
title: handle_project_close_confirm
description: "Resolve the user's Save All / Discard All / Cancel choice on"
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/close_project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/close_project/handle_project_close_confirm
language: rust
---

# handle_project_close_confirm

Resolve the user's Save All / Discard All / Cancel choice on

## Signature

```rust
impl Oxide { pub(crate) fn handle_project_close_confirm(
        &mut self,
        choice: crate::app::ProjectCloseChoice,
    ) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Docstring

Resolve the user's Save All / Discard All / Cancel choice on
the project-close confirmation modal. Owned by this module
because it's a follow-up of `close_project_at_tree_path`.

## Source
Lines 213–279 in `crates/oxide-app/src/app/handlers/dock/project_navigation/close_project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [close_project](/crates/oxide-app/src/app/handlers/dock/project_navigation/close_project.md) |
| calls | [log_error](/crates/oxide-app/src/diagnostics/log_error.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
