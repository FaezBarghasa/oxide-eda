---
okf_version: "0.2"
type: Function
title: on_primary_pressed
description: Left press — walk the per-tool click arms in order. First one
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/on_primary_pressed
language: rust
---

# on_primary_pressed

Left press — walk the per-tool click arms in order. First one

## Signature

```rust
impl FootprintCanvas<'_> { fn on_primary_pressed(
        &self,
        cstate: &mut FootprintCanvasState,
        cursor_pos: Point,
    ) -> Option<canvas::Action<LibraryMessage>> }
```

## Docstring

Left press — walk the per-tool click arms in order. First one
to claim the click returns; the empty-canvas tail arms the
pending drag + rubber-band.

## Source
Lines 131–169 in `crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pointer](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.md) |
