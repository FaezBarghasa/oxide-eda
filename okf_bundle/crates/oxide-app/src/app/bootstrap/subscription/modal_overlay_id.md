---
okf_version: "0.2"
type: Function
title: modal_overlay_id
description: Which overlay slot a detachable modal occupies in the main window
resource: crates/oxide-app/src/app/bootstrap/subscription.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/bootstrap/subscription/modal_overlay_id
language: rust
---

# modal_overlay_id

Which overlay slot a detachable modal occupies in the main window

## Signature

```rust
fn modal_overlay_id(modal: crate::app::state::ModalId) -> Option<OverlayId>
```

## Docstring

Which overlay slot a detachable modal occupies in the main window
(#547).

The pairing exists so an Esc typed in a detached window can be
answered from the SAME rung table the in-window card resolves
against, instead of a second list of Close messages that would drift
out of step with it. Exhaustive over `ModalId`, so a new detachable
modal is a compile error here rather than a silent fall-through to
the main window's tool reset.

`None` is not an omission: `MoveSelection`, `NetColorPalette` and
`ParameterManager` have no `OverlayId` and no `OpenOverlays` field —
they are part of the `every_modal_claims_escape` gap, and Esc does
nothing over their in-window cards either.

## Source
Lines 353–371 in `crates/oxide-app/src/app/bootstrap/subscription.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [subscription](/crates/oxide-app/src/app/bootstrap/subscription.md) |
| called_by | [detached_modal_escape_message](/crates/oxide-app/src/app/bootstrap/subscription/detached_modal_escape_message.md) |
