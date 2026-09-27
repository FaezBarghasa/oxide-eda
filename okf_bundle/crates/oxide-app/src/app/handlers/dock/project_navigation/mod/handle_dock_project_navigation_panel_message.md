---
okf_version: "0.2"
type: Function
title: handle_dock_project_navigation_panel_message
description: "Returns `None` when the message isn't a project-navigation message"
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/mod/handle_dock_project_navigation_panel_message
language: rust
---

# handle_dock_project_navigation_panel_message

Returns `None` when the message isn't a project-navigation message

## Signature

```rust
impl Oxide { pub(super) fn handle_dock_project_navigation_panel_message(
        &mut self,
        panel_msg: &crate::panels::PanelMsg,
    ) -> Option<Task<Message>> }
```

## Visibility

- `pub(super)`

## Docstring

Returns `None` when the message isn't a project-navigation message
(so the caller falls through to the next dock handler), or
`Some(task)` when handled — the task carries the follow-up work
from opening a project-tree document (library-browser mount /
primitive-editor open) so it isn't dropped.

## Source
Lines 19–106 in `crates/oxide-app/src/app/handlers/dock/project_navigation/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_navigation](/crates/oxide-app/src/app/handlers/dock/project_navigation/mod.md) |
| calls | [get_node](/crates/oxide-widgets/src/tree_view/get_node.md) |
| calls | [log_error](/crates/oxide-app/src/diagnostics/log_error.md) |
