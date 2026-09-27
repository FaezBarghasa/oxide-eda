---
okf_version: "0.2"
type: Function
title: released_sketch_click
description: Sketch-mode empty-canvas click-add — route to the active sketch
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/release.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/release/released_sketch_click_1
language: rust
---

# released_sketch_click

Sketch-mode empty-canvas click-add — route to the active sketch

## Signature

```rust
fn released_sketch_click(
        &self,
        cstate: &FootprintCanvasState,
        drag: &DragState,
    ) -> Option<canvas::Action<LibraryMessage>>
```

## Docstring

Sketch-mode empty-canvas click-add — route to the active sketch
tool (Select / Point / Line / … multi-click gestures), honouring
TAB placement-pause.

## Source
Lines 266–325 in `crates/oxide-app/src/library/editor/footprint/canvas/input/release.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [release](/crates/oxide-app/src/library/editor/footprint/canvas/input/release.md) |
| calls | [sketch_snap](/crates/oxide-app/src/library/editor/footprint/canvas/hit_test/sketch_snap.md) |
| calls | [sketch_hit_other](/crates/oxide-app/src/library/editor/footprint/canvas/hit_test/sketch_hit_other.md) |
