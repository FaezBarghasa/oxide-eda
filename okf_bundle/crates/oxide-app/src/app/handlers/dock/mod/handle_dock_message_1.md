---
okf_version: "0.2"
type: Function
title: handle_dock_message
resource: crates/oxide-app/src/app/handlers/dock/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/mod/handle_dock_message_1
language: rust
---

# handle_dock_message

## Signature

```rust
pub(crate) fn handle_dock_message(&mut self, msg: DockMessage) -> Task<Message>
```

## Visibility

- `pub(crate)`

## Source
Lines 16–56 in `crates/oxide-app/src/app/handlers/dock/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dock](/crates/oxide-app/src/app/handlers/dock/mod.md) |
| calls | [write_dock_layout](/crates/oxide-app/src/fonts/dock_layout/write_dock_layout.md) |
