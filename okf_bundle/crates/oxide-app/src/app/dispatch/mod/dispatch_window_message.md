---
okf_version: "0.2"
type: Function
title: dispatch_window_message
description: "Window lifecycle, docking, and native-chrome message family"
resource: crates/oxide-app/src/app/dispatch/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/mod/dispatch_window_message
language: rust
---

# dispatch_window_message

Window lifecycle, docking, and native-chrome message family

## Signature

```rust
impl Oxide { pub(crate) fn dispatch_window_message(&mut self, message: WindowMsg) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Docstring

Window lifecycle, docking, and native-chrome message family
(namespaced, ADR-0001 D3). Main + secondary window open / close /
resize / scale, detach + reattach of modals, undocked tabs and
floating panels, and the borderless-chrome drag / resize / minimize
/ maximize buttons. Routed from `dispatch_update`.

## Source
Lines 250–561 in `crates/oxide-app/src/app/dispatch/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dispatch](/crates/oxide-app/src/app/dispatch/mod.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| calls | [write_dock_layout](/crates/oxide-app/src/fonts/dock_layout/write_dock_layout.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [close](/crates/oxide-app/src/library/editor/footprint/updates/context_menu/close.md) |
| calls | [start_window_drag](/crates/oxide-app/src/chrome/start_window_drag.md) |
| calls | [start_window_resize](/crates/oxide-app/src/chrome/start_window_resize.md) |
