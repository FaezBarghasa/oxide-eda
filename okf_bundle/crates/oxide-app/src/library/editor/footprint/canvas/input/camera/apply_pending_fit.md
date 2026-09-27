---
okf_version: "0.2"
type: Function
title: apply_pending_fit
description: v0.26-C — one-shot Fit-to-Window from the right-click menu.
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/camera.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/camera/apply_pending_fit
language: rust
---

# apply_pending_fit

v0.26-C — one-shot Fit-to-Window from the right-click menu.

## Signature

```rust
impl FootprintCanvas<'_> { pub(in crate::library::editor::footprint::canvas) fn apply_pending_fit(
        &self,
        cstate: &mut FootprintCanvasState,
        bounds: Rectangle,
    ) -> Option<canvas::Action<LibraryMessage>> }
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

v0.26-C — one-shot Fit-to-Window from the right-click menu.
The dispatcher set `state.fit_pending = true`; we honour it
on the very next event tick (any mouse motion / scroll /
press over the canvas) and publish `FitConsumed` so the
flag clears. Bounds-guard mirrors the first-draw branch.

## Source
Lines 48–69 in `crates/oxide-app/src/library/editor/footprint/canvas/input/camera.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [camera](/crates/oxide-app/src/library/editor/footprint/canvas/input/camera.md) |
