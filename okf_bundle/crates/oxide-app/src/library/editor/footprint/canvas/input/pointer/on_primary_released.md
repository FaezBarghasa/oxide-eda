---
okf_version: "0.2"
type: Function
title: on_primary_released
description: "Left release — take the drag, dispatch by its kind (round-pad"
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/on_primary_released
language: rust
---

# on_primary_released

Left release — take the drag, dispatch by its kind (round-pad

## Signature

```rust
impl FootprintCanvas<'_> { fn on_primary_released(
        &self,
        cstate: &mut FootprintCanvasState,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<LibraryMessage>> }
```

## Docstring

Left release — take the drag, dispatch by its kind (round-pad
resize / sketch-point / empty-or-tool / pad-drag settle).

## Source
Lines 313–349 in `crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pointer](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.md) |
