---
okf_version: "0.2"
type: Function
title: handle_active_bar_message
resource: crates/oxide-app/src/app/handlers/active_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/active_bar/mod/handle_active_bar_message_1
language: rust
---

# handle_active_bar_message

## Signature

```rust
pub(crate) fn handle_active_bar_message(
        &mut self,
        msg: crate::active_bar::ActiveBarMsg,
    ) -> Task<Message>
```

## Visibility

- `pub(crate)`

## Source
Lines 10–31 in `crates/oxide-app/src/app/handlers/active_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/app/handlers/active_bar/mod.md) |
