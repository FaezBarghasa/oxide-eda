---
okf_version: "0.2"
type: Function
title: handle_modal_drag_start
resource: crates/oxide-app/src/app/handlers/erc/modals.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/erc/modals/handle_modal_drag_start_1
language: rust
---

# handle_modal_drag_start

## Signature

```rust
pub(crate) fn handle_modal_drag_start(
        &mut self,
        modal: super::super::super::state::ModalId,
        x: f32,
        y: f32,
    ) -> Task<Message>
```

## Visibility

- `pub(crate)`

## Source
Lines 46–54 in `crates/oxide-app/src/app/handlers/erc/modals.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [modals](/crates/oxide-app/src/app/handlers/erc/modals.md) |
