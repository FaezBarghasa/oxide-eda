---
okf_version: "0.2"
type: Function
title: library_picker_overlay
description: v0.9 Library — picker modal overlay. Centered card on a dim
resource: crates/oxide-app/src/app/view/overlays/modals.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/modals/library_picker_overlay_1
language: rust
---

# library_picker_overlay

v0.9 Library — picker modal overlay. Centered card on a dim

## Signature

```rust
pub(in crate::app::view) fn library_picker_overlay(&self) -> Option<Element<'_, Message>>
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

v0.9 Library — picker modal overlay. Centered card on a dim
backdrop; Esc dismisses via `OpenOverlays::escape_message`'s
`library_picker_open` rung (`app/bootstrap/subscription.rs`), the
close X is the mouse-click path.

## Source
Lines 233–250 in `crates/oxide-app/src/app/view/overlays/modals.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [modals](/crates/oxide-app/src/app/view/overlays/modals.md) |
