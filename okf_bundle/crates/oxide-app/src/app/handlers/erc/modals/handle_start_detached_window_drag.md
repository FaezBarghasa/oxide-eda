---
okf_version: "0.2"
type: Function
title: handle_start_detached_window_drag
description: Ask the OS to start a borderless-window drag for whichever window
resource: crates/oxide-app/src/app/handlers/erc/modals.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/erc/modals/handle_start_detached_window_drag
language: rust
---

# handle_start_detached_window_drag

Ask the OS to start a borderless-window drag for whichever window

## Signature

```rust
impl Oxide { pub(crate) fn handle_start_detached_window_drag(
        &mut self,
        modal: super::super::super::state::ModalId,
    ) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Docstring

Ask the OS to start a borderless-window drag for whichever window
currently hosts `modal`. Wired to the decorations:false detached
modal header so the user can move the window without an OS
title bar.

## Source
Lines 366–382 in `crates/oxide-app/src/app/handlers/erc/modals.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [modals](/crates/oxide-app/src/app/handlers/erc/modals.md) |
| calls | [start_window_drag](/crates/oxide-app/src/chrome/start_window_drag.md) |
