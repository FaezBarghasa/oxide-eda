---
okf_version: "0.2"
type: Function
title: handle_remove_confirm
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/remove.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/remove/handle_remove_confirm_1
language: rust
---

# handle_remove_confirm

## Signature

```rust
pub(crate) fn handle_remove_confirm(
        &mut self,
        choice: crate::app::RemoveChoice,
    ) -> iced::Task<Message>
```

## Visibility

- `pub(crate)`

## Source
Lines 26–159 in `crates/oxide-app/src/app/handlers/dock/project_navigation/remove.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [remove](/crates/oxide-app/src/app/handlers/dock/project_navigation/remove.md) |
| calls | [log_error](/crates/oxide-app/src/diagnostics/log_error.md) |
