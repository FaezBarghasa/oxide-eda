---
okf_version: "0.2"
type: Function
title: detached_modal_resize_overlay
description: "Same 6 px edge-resize overlay as the main window's, but"
resource: crates/oxide-app/src/app/view/chrome.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/chrome/detached_modal_resize_overlay
language: rust
---

# detached_modal_resize_overlay

Same 6 px edge-resize overlay as the main window's, but

## Signature

```rust
impl Oxide { fn detached_modal_resize_overlay(modal: super::state::ModalId) -> Element<'a, Message> }
```

## Type Parameters

- `'a`

## Docstring

Same 6 px edge-resize overlay as the main window's, but
emitting `StartDetachedModalResize { modal, direction }`
so it dispatches to the right OS window. Used as a stack
layer above the modal's body in `view_detached_modal`.

## Source
Lines 374–441 in `crates/oxide-app/src/app/view/chrome.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [chrome](/crates/oxide-app/src/app/view/chrome.md) |
