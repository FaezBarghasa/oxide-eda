---
okf_version: "0.2"
type: Function
title: check_floating_panel_auto_detach
description: Scan the floating-panel list for one whose drag just crossed the
resource: crates/oxide-app/src/app/handlers/canvas/layout_drag.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/canvas/layout_drag/check_floating_panel_auto_detach_1
language: rust
---

# check_floating_panel_auto_detach

Scan the floating-panel list for one whose drag just crossed the

## Signature

```rust
pub(crate) fn check_floating_panel_auto_detach(
        &self,
        cursor_x: f32,
        cursor_y: f32,
    ) -> Option<usize>
```

## Visibility

- `pub(crate)`

## Docstring

Scan the floating-panel list for one whose drag just crossed the
main window boundary. Returns the index into `dock.floating` so
the dispatcher can chain a `DetachFloatingPanel(idx)` task.

## Source
Lines 146–165 in `crates/oxide-app/src/app/handlers/canvas/layout_drag.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [layout_drag](/crates/oxide-app/src/app/handlers/canvas/layout_drag.md) |
