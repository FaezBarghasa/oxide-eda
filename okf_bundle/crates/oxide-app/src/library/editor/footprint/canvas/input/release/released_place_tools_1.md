---
okf_version: "0.2"
type: Function
title: released_place_tools
description: Pads-mode empty-canvas click — place pad / via / hole / string /
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/release.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/release/released_place_tools_1
language: rust
---

# released_place_tools

Pads-mode empty-canvas click — place pad / via / hole / string /

## Signature

```rust
fn released_place_tools(
        &self,
        cstate: &mut FootprintCanvasState,
        drag: &DragState,
    ) -> Option<canvas::Action<LibraryMessage>>
```

## Docstring

Pads-mode empty-canvas click — place pad / via / hole / string /
track / arc / polygon / region (gated on `placement_paused`) or,
for the Select tool, clear the current selection.

## Source
Lines 330–445 in `crates/oxide-app/src/library/editor/footprint/canvas/input/release.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [release](/crates/oxide-app/src/library/editor/footprint/canvas/input/release.md) |
