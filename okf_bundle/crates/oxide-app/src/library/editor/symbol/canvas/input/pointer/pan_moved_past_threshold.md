---
okf_version: "0.2"
type: Function
title: pan_moved_past_threshold
description: "Whether cumulative displacement from the fixed press `origin` to"
resource: crates/oxide-app/src/library/editor/symbol/canvas/input/pointer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/pan_moved_past_threshold
language: rust
---

# pan_moved_past_threshold

Whether cumulative displacement from the fixed press `origin` to

## Signature

```rust
fn pan_moved_past_threshold(origin: iced::Point, current: iced::Point) -> bool
```

## Docstring

Whether cumulative displacement from the fixed press `origin` to
`current` crosses the pan motion threshold that latches
`CanvasState::pan_moved` (see `on_cursor_moved`). Comparing against
the ORIGIN, not the per-frame delta, means a slow, deliberate drag
— many sub-threshold per-frame steps — is still recognised as a
real pan once its total displacement adds up past the threshold.

## Source
Lines 379–383 in `crates/oxide-app/src/library/editor/symbol/canvas/input/pointer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pointer](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer.md) |
| called_by | [on_cursor_moved](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/on_cursor_moved.md) |
