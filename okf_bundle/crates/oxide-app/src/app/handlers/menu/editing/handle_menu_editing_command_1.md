---
okf_version: "0.2"
type: Function
title: handle_menu_editing_command
resource: crates/oxide-app/src/app/handlers/menu/editing.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/menu/editing/handle_menu_editing_command_1
language: rust
---

# handle_menu_editing_command

## Signature

```rust
pub(super) fn handle_menu_editing_command(
        &mut self,
        msg: &MenuMessage,
    ) -> Option<Task<Message>>
```

## Visibility

- `pub(super)`

## Source
Lines 6–66 in `crates/oxide-app/src/app/handlers/menu/editing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [editing](/crates/oxide-app/src/app/handlers/menu/editing.md) |
| calls | [log_info](/crates/oxide-app/src/diagnostics/log_info.md) |
