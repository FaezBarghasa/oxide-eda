---
okf_version: "0.2"
type: Function
title: on_secondary_released
description: Right/Middle release — a right-release that did not pan opens
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/on_secondary_released_1
language: rust
---

# on_secondary_released

Right/Middle release — a right-release that did not pan opens

## Signature

```rust
fn on_secondary_released(
        &self,
        cstate: &mut FootprintCanvasState,
        button: &mouse::Button,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<LibraryMessage>>
```

## Docstring

Right/Middle release — a right-release that did not pan opens
the context menu (pad → silk → empty hit priority).

## Source
Lines 259–309 in `crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pointer](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.md) |
| calls | [silk_f_hit_at](/crates/oxide-app/src/library/editor/footprint/canvas/mod/silk_f_hit_at.md) |
