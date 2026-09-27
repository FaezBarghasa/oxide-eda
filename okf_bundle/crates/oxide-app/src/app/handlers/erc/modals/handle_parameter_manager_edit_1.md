---
okf_version: "0.2"
type: Function
title: handle_parameter_manager_edit
resource: crates/oxide-app/src/app/handlers/erc/modals.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/erc/modals/handle_parameter_manager_edit_1
language: rust
---

# handle_parameter_manager_edit

## Signature

```rust
pub(crate) fn handle_parameter_manager_edit(
        &mut self,
        symbol_uuid: uuid::Uuid,
        key: String,
        value: String,
    ) -> Task<Message>
```

## Visibility

- `pub(crate)`

## Source
Lines 339–360 in `crates/oxide-app/src/app/handlers/erc/modals.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [modals](/crates/oxide-app/src/app/handlers/erc/modals.md) |
