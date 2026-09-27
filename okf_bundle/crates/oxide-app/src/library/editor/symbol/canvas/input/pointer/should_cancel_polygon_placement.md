---
okf_version: "0.2"
type: Function
title: should_cancel_polygon_placement
description: Whether a secondary-button press should cancel the Place Polygon
resource: crates/oxide-app/src/library/editor/symbol/canvas/input/pointer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/should_cancel_polygon_placement
language: rust
---

# should_cancel_polygon_placement

Whether a secondary-button press should cancel the Place Polygon

## Signature

```rust
fn should_cancel_polygon_placement(
    button: mouse::Button,
    tool: SymbolTool,
    polygon_vertices_empty: bool,
) -> bool
```

## Docstring

Whether a secondary-button press should cancel the Place Polygon
stash instead of arming a pan (see `on_secondary_press`). Only
`Right` cancels — the stash has no undo, so a `Middle`-button pan
attempt must never destroy it.

## Source
Lines 365–371 in `crates/oxide-app/src/library/editor/symbol/canvas/input/pointer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pointer](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer.md) |
| called_by | [on_secondary_press](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/on_secondary_press.md) |
