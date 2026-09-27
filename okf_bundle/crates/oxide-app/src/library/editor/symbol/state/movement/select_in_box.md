---
okf_version: "0.2"
type: Function
title: select_in_box
description: Perform a rubber-band box selection against all symbol primitives.
resource: crates/oxide-app/src/library/editor/symbol/state/movement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/movement/select_in_box
language: rust
---

# select_in_box

Perform a rubber-band box selection against all symbol primitives.

## Signature

```rust
pub fn select_in_box(
    sym: &Symbol,
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    kind: BoxSelectKind,
    active_part: u8,
) -> Option<SymbolSelection>
```

## Visibility

- `pub`

## Docstring

Perform a rubber-band box selection against all symbol primitives.

The selection kind (`Window` / `Crossing`) is determined by the
caller from the drag direction before calling this function.
Returns `None` when nothing falls inside the box.

## Source
Lines 186–269 in `crates/oxide-app/src/library/editor/symbol/state/movement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [movement](/crates/oxide-app/src/library/editor/symbol/state/movement.md) |
| calls | [pin_on_part](/crates/oxide-app/src/library/editor/symbol/state/mod/pin_on_part.md) |
| calls | [point_in_box](/crates/oxide-app/src/library/editor/symbol/state/movement/point_in_box.md) |
| calls | [pin_body_delta](/crates/oxide-app/src/library/editor/symbol/state/rotation/pin_body_delta.md) |
| calls | [segment_crosses_box](/crates/oxide-app/src/library/editor/symbol/state/movement/segment_crosses_box.md) |
| calls | [graphic_on_part](/crates/oxide-app/src/library/editor/symbol/state/mod/graphic_on_part.md) |
| calls | [graphic_fully_inside_box](/crates/oxide-app/src/library/editor/symbol/state/movement/graphic_fully_inside_box.md) |
| calls | [graphic_intersects_box](/crates/oxide-app/src/library/editor/symbol/state/movement/graphic_intersects_box.md) |
| called_by | [on_left_release](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/on_left_release.md) |
| called_by | [select_in_box_all_uses_visible_counts](/crates/oxide-app/src/library/editor/symbol/state/tests/select_in_box_all_uses_visible_counts.md) |
