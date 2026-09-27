---
okf_version: "0.2"
type: Function
title: handle_detach_modal
description: "Pop `modal` out of the main window into its own OS window. The"
resource: crates/oxide-app/src/app/handlers/erc/modals.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/erc/modals/handle_detach_modal
language: rust
---

# handle_detach_modal

Pop `modal` out of the main window into its own OS window. The

## Signature

```rust
impl Oxide { pub(crate) fn handle_detach_modal(
        &mut self,
        modal: super::super::super::state::ModalId,
    ) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Docstring

Pop `modal` out of the main window into its own OS window. The
window's initial size matches the modal's in-app dimensions so the
user sees continuity; position falls back to default (centered on
the OS) since we don't know where to anchor absent monitor query.

## Source
Lines 208–284 in `crates/oxide-app/src/app/handlers/erc/modals.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [modals](/crates/oxide-app/src/app/handlers/erc/modals.md) |
