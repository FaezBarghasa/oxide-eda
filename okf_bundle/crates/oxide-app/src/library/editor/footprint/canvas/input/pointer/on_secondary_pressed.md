---
okf_version: "0.2"
type: Function
title: on_secondary_pressed
description: "Right/Middle press — right-click tool cancel (schematic parity),"
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/on_secondary_pressed
language: rust
---

# on_secondary_pressed

Right/Middle press — right-click tool cancel (schematic parity),

## Signature

```rust
impl FootprintCanvas<'_> { fn on_secondary_pressed(
        &self,
        cstate: &mut FootprintCanvasState,
        button: &mouse::Button,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<LibraryMessage>> }
```

## Docstring

Right/Middle press — right-click tool cancel (schematic parity),
otherwise start a pan.

## Source
Lines 45–126 in `crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pointer](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.md) |
