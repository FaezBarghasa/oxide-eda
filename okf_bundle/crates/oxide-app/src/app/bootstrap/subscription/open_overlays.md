---
okf_version: "0.2"
type: Function
title: open_overlays
description: "Snapshot the open overlays for Esc resolution, from live state."
resource: crates/oxide-app/src/app/bootstrap/subscription.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/bootstrap/subscription/open_overlays
language: rust
---

# open_overlays

Snapshot the open overlays for Esc resolution, from live state.

## Signature

```rust
impl Oxide { fn open_overlays(&self, esc_from_detached: Option<crate::app::state::ModalId>) -> OpenOverlays }
```

## Docstring

Snapshot the open overlays for Esc resolution, from live state.

Called from the `Message::EscapePressed` handler in
`app/dispatch/mod.rs`, not from the subscription (#535). Building it
here rather than inside `Subscription::with` is what lets a rung
carry owned data (see `OpenOverlays::delete_confirm`) and what
removes the one-update staleness the subscription snapshot had.

`esc_from_detached` names the modal whose own detached window the
Esc came from, if any — see `detached_modal_escape_message`. Every
other caller passes `None`.

## Source
Lines 423–508 in `crates/oxide-app/src/app/bootstrap/subscription.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [subscription](/crates/oxide-app/src/app/bootstrap/subscription.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
