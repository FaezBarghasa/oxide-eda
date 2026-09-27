---
okf_version: "0.2"
type: Function
title: handle_project_tree_action
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/mod/handle_project_tree_action
language: rust
---

# handle_project_tree_action

## Signature

```rust
impl Oxide { pub(crate) fn handle_project_tree_action(
        &mut self,
        action: crate::app::ProjectTreeAction,
    ) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Source
Lines 115–227 in `crates/oxide-app/src/app/handlers/dock/project_navigation/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_navigation](/crates/oxide-app/src/app/handlers/dock/project_navigation/mod.md) |
| calls | [set_expanded_recursive](/crates/oxide-app/src/app/handlers/dock/project_navigation/mod/set_expanded_recursive.md) |
| calls | [build_project_tree](/crates/oxide-app/src/panels/projects/build_project_tree.md) |
| calls | [reveal_in_file_manager](/crates/oxide-app/src/app/handlers/dock/project_navigation/mod/reveal_in_file_manager.md) |
| calls | [log_error](/crates/oxide-app/src/diagnostics/log_error.md) |
