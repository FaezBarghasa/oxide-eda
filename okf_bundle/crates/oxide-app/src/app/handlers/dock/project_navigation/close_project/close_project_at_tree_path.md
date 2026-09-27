---
okf_version: "0.2"
type: Function
title: close_project_at_tree_path
description: Close every tab backed by the project whose root is at
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/close_project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/close_project/close_project_at_tree_path
language: rust
---

# close_project_at_tree_path

Close every tab backed by the project whose root is at

## Signature

```rust
impl Oxide { pub(super) fn close_project_at_tree_path(&mut self, tree_path: &[usize]) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Docstring

Close every tab backed by the project whose root is at
`tree_path[0]`, then drop the project from the workspace and
promote a sibling (or `None`) to active. Mirrors Altium's
Projects-panel right-click → Close Project.

If any file in the project's directory has unsaved edits
(`dirty_paths` intersects the project dir), opens the
`ProjectCloseConfirm` modal first instead of closing
immediately. The modal's choice handler
(`handle_project_close_confirm`) calls back into this method
once the user picks Save All or Discard All.

## Source
Lines 21–64 in `crates/oxide-app/src/app/handlers/dock/project_navigation/close_project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [close_project](/crates/oxide-app/src/app/handlers/dock/project_navigation/close_project.md) |
