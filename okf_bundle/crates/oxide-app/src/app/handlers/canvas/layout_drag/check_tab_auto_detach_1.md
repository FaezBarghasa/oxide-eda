---
okf_version: "0.2"
type: Function
title: check_tab_auto_detach
description: Returns the tab index to undock when the user is dragging a
resource: crates/oxide-app/src/app/handlers/canvas/layout_drag.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/canvas/layout_drag/check_tab_auto_detach_1
language: rust
---

# check_tab_auto_detach

Returns the tab index to undock when the user is dragging a

## Signature

```rust
pub(crate) fn check_tab_auto_detach(&self, cursor_x: f32, cursor_y: f32) -> Option<usize>
```

## Visibility

- `pub(crate)`

## Docstring

Returns the tab index to undock when the user is dragging a
document tab and the cursor crosses the main window boundary.

## Source
Lines 125–141 in `crates/oxide-app/src/app/handlers/canvas/layout_drag.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [layout_drag](/crates/oxide-app/src/app/handlers/canvas/layout_drag.md) |
