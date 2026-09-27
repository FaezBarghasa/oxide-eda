---
okf_version: "0.2"
type: Function
title: handle_canvas_event_in_window
description: "Handle a `CanvasEvent` that originated in a non-main window."
resource: crates/oxide-app/src/app/dispatch/ui.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/ui/handle_canvas_event_in_window_1
language: rust
---

# handle_canvas_event_in_window

Handle a `CanvasEvent` that originated in a non-main window.

## Signature

```rust
pub(super) fn handle_canvas_event_in_window(
        &mut self,
        window_id: iced::window::Id,
        event: crate::canvas::CanvasEvent,
    ) -> iced::Task<Message>
```

## Visibility

- `pub(super)`

## Docstring

Handle a `CanvasEvent` that originated in a non-main window.

The canvas event handlers assume `self.interaction_state.canvas`
is the live target. To avoid rewriting hundreds of call sites,
we temporarily swap the per-window canvas into the main slot
(and point `document_state.active_path` at the window's tab
path so engine lookups resolve correctly), run the handler, and
swap back. Writes to other sub-fields of `interaction_state` /
`document_state` still occur — the user experience is that the
non-main window behaves like "the active window" for the
duration of its event.

**Invariant:** any `Task<Message>` returned by the handler runs
AFTER the swap unwinds. If a handler ever chains
`Task::done(Message::CanvasEvent(..))` expecting to land back
in the window that produced the original event, it must use
`Message::CanvasEventInWindow { window_id, .. }` instead — the
plain `CanvasEvent` form always targets the main window.

**Panic safety:** the swap is guarded with `catch_unwind` +
`resume_unwind` so a panicking handler still restores both the
canvas slot and `active_path` before the panic propagates.

## Source
Lines 366–438 in `crates/oxide-app/src/app/dispatch/ui.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ui](/crates/oxide-app/src/app/dispatch/ui.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
