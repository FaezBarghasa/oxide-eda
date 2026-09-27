---
okf_version: "0.2"
type: Function
title: box_select_pads
description: Pads-mode rubber-band picker — honour the active-bar Selection
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/release.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/release/box_select_pads_1
language: rust
---

# box_select_pads

Pads-mode rubber-band picker — honour the active-bar Selection

## Signature

```rust
fn box_select_pads(
        &self,
        cstate: &FootprintCanvasState,
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
    ) -> Option<canvas::Action<LibraryMessage>>
```

## Docstring

Pads-mode rubber-band picker — honour the active-bar Selection
mode (Inside / Touching / Outside) and combine with the current
selection under Ctrl/Shift.

## Source
Lines 196–261 in `crates/oxide-app/src/library/editor/footprint/canvas/input/release.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [release](/crates/oxide-app/src/library/editor/footprint/canvas/input/release.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
