---
okf_version: "0.2"
type: Function
title: canvas_for_window
description: "Per-window canvas lookup. Returns the per-window `CanvasSlot`"
resource: crates/oxide-app/src/app/state/interaction.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/state/interaction/canvas_for_window_1
language: rust
---

# canvas_for_window

Per-window canvas lookup. Returns the per-window `CanvasSlot`

## Signature

```rust
pub fn canvas_for_window(&self, window_id: iced::window::Id) -> &CanvasSlot
```

## Visibility

- `pub`

## Docstring

Per-window canvas lookup. Returns the per-window `CanvasSlot`
if one is registered (undocked windows), otherwise the main
window's shared canvas. Writes from canvas events still go
through the main-canvas slot; see the dispatch swap trick in
`dispatch::ui::handle_canvas_event_in_window`.

## Source
Lines 149–151 in `crates/oxide-app/src/app/state/interaction.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [interaction](/crates/oxide-app/src/app/state/interaction.md) |
