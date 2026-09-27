---
okf_version: "0.2"
type: Function
title: check_modal_auto_detach
description: "Altium-style auto-detach. While the user drags a modal's title"
resource: crates/oxide-app/src/app/handlers/canvas/layout_drag.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/canvas/layout_drag/check_modal_auto_detach
language: rust
---

# check_modal_auto_detach

Altium-style auto-detach. While the user drags a modal's title

## Signature

```rust
impl Oxide { pub(crate) fn check_modal_auto_detach(
        &self,
        cursor_x: f32,
        cursor_y: f32,
    ) -> Option<super::super::super::state::ModalId> }
```

## Visibility

- `pub(crate)`

## Docstring

Altium-style auto-detach. While the user drags a modal's title
bar, watch the cursor; if it crosses the main window boundary by
more than `EDGE_THRESHOLD`, pop the modal out into its own OS
window. Returns the modal that should detach, if any, so the
dispatcher can chain a `DetachModal` task onto the DragMove path.

## Source
Lines 172–199 in `crates/oxide-app/src/app/handlers/canvas/layout_drag.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [layout_drag](/crates/oxide-app/src/app/handlers/canvas/layout_drag.md) |
