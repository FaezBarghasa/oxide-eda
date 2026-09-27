---
okf_version: "0.2"
type: Function
title: released_box_select
description: Rubber-band commit — derive the world-space rectangle from the
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/release.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/release/released_box_select
language: rust
---

# released_box_select

Rubber-band commit — derive the world-space rectangle from the

## Signature

```rust
impl FootprintCanvas<'_> { fn released_box_select(
        &self,
        cstate: &FootprintCanvasState,
        a: Point,
        c: Point,
    ) -> Option<canvas::Action<LibraryMessage>> }
```

## Docstring

Rubber-band commit — derive the world-space rectangle from the
press/current screen anchors, then pick sketch entities (Sketch
mode) or pads (otherwise).

## Source
Lines 104–131 in `crates/oxide-app/src/library/editor/footprint/canvas/input/release.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [release](/crates/oxide-app/src/library/editor/footprint/canvas/input/release.md) |
