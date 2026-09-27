---
okf_version: "0.2"
type: Function
title: try_round_resize_tick
description: v0.27 — round-pad diameter handle drag tick.
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/try_round_resize_tick_1
language: rust
---

# try_round_resize_tick

v0.27 — round-pad diameter handle drag tick.

## Signature

```rust
fn try_round_resize_tick(
        &self,
        cstate: &FootprintCanvasState,
        world: (f64, f64),
    ) -> Option<canvas::Action<LibraryMessage>>
```

## Docstring

v0.27 — round-pad diameter handle drag tick.

## Source
Lines 424–449 in `crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pointer](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.md) |
