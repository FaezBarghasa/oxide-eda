---
okf_version: "0.2"
type: Function
title: close_detached_modal
description: "Find any OS window that currently hosts `modal` and request the"
resource: crates/oxide-app/src/app/handlers/erc/modals.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/erc/modals/close_detached_modal
language: rust
---

# close_detached_modal

Find any OS window that currently hosts `modal` and request the

## Signature

```rust
impl Oxide { pub(crate) fn close_detached_modal(
        &mut self,
        modal: super::super::super::state::ModalId,
    ) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Docstring

Find any OS window that currently hosts `modal` and request the
OS to close it. Used by the in-body Close button so pressing Close
inside a detached modal both dismisses the modal state and cleans
up the popped-out window — without this, the window would stay
open rendering an orphaned modal body.

## Source
Lines 184–202 in `crates/oxide-app/src/app/handlers/erc/modals.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [modals](/crates/oxide-app/src/app/handlers/erc/modals.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| calls | [close](/crates/oxide-app/src/library/editor/footprint/updates/context_menu/close.md) |
