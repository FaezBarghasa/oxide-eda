---
okf_version: "0.2"
type: Function
title: handle_active_bar_action
resource: crates/oxide-app/src/app/handlers/active_bar/action_groups.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/active_bar/action_groups/handle_active_bar_action_1
language: rust
---

# handle_active_bar_action

## Signature

```rust
pub(crate) fn handle_active_bar_action(
        &mut self,
        action: crate::active_bar::ActiveBarAction,
    ) -> Task<Message>
```

## Visibility

- `pub(crate)`

## Source
Lines 6–309 in `crates/oxide-app/src/app/handlers/active_bar/action_groups.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [action_groups](/crates/oxide-app/src/app/handlers/active_bar/action_groups.md) |
| calls | [Tool](/crates/oxide-app/src/app/documents/Tool.md) |
| calls | [log_info](/crates/oxide-app/src/diagnostics/log_info.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
