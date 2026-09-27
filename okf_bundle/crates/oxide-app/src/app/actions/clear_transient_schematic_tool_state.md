---
okf_version: "0.2"
type: Function
title: clear_transient_schematic_tool_state
description: Cancel the in-flight placement session and every armed mode.
resource: crates/oxide-app/src/app/actions.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/actions/clear_transient_schematic_tool_state
language: rust
---

# clear_transient_schematic_tool_state

Cancel the in-flight placement session and every armed mode.

## Signature

```rust
impl Oxide { pub(crate) fn clear_transient_schematic_tool_state(&mut self) }
```

## Visibility

- `pub(crate)`

## Docstring

Cancel the in-flight placement session and every armed mode.

There is exactly ONE session app-wide — `current_tool` and all the
placement buffers below are single fields, and a tool picked from
an undocked window's toolbar sets those same globals — so the
session half of this runs once. The canvas half does not: the
ghosts and previews live on each window's `CanvasSlot`.

#554 — this used to reach the canvas only through
`active_canvas_mut()`, which is always the MAIN slot
(`state/interaction.rs`). The per-window canvases in
`InteractionState::canvases` were never swept, and the
wire-preview paint gate is two per-canvas fields
(`if self.drawing_mode && !self.wire_preview.is_empty()`,
`canvas/draw/previews.rs`) — so cancelling a wire left an undocked
window painting a frozen ghost of it. Sweeping every canvas fixes
that in both directions: Esc in the undocked window, AND Esc in
the main window while the wire was being drawn in the undocked
one, which per-window routing alone would not.

## Source
Lines 217–272 in `crates/oxide-app/src/app/actions.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [actions](/crates/oxide-app/src/app/actions.md) |
