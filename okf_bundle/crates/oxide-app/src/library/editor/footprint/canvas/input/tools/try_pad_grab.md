---
okf_version: "0.2"
type: Function
title: try_pad_grab
description: v0.21/v0.23/v0.27 — Pad hit (Pads mode + pads filter on). Starts
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/tools.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_pad_grab
language: rust
---

# try_pad_grab

v0.21/v0.23/v0.27 — Pad hit (Pads mode + pads filter on). Starts

## Signature

```rust
impl FootprintCanvas<'_> { pub(in crate::library::editor::footprint::canvas) fn try_pad_grab(
        &self,
        cstate: &mut FootprintCanvasState,
        cursor_pos: Point,
        world: (f64, f64),
    ) -> Option<canvas::Action<LibraryMessage>> }
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

v0.21/v0.23/v0.27 — Pad hit (Pads mode + pads filter on). Starts
a pad drag and publishes the selection, honouring Ctrl/Cmd
toggle + Shift extend modifiers.

## Source
Lines 430–488 in `crates/oxide-app/src/library/editor/footprint/canvas/input/tools.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tools](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
