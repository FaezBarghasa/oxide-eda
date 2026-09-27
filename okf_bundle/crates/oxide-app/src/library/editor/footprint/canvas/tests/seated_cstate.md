---
okf_version: "0.2"
type: Function
title: seated_cstate
description: "Deterministic canvas transform (scale 10 px/mm, world origin at"
resource: crates/oxide-app/src/library/editor/footprint/canvas/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/tests/seated_cstate
language: rust
---

# seated_cstate

Deterministic canvas transform (scale 10 px/mm, world origin at

## Signature

```rust
fn seated_cstate() -> FootprintCanvasState
```

## Docstring

Deterministic canvas transform (scale 10 px/mm, world origin at
screen (100,300)) so a chosen world point maps to a screen cursor
that lands inside the test bounds and inverts back exactly.

## Source
Lines 298–304 in `crates/oxide-app/src/library/editor/footprint/canvas/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/footprint/canvas/tests.md) |
| called_by | [armed_drag_track_end_wins_walk_order_via_dispatcher](/crates/oxide-app/src/library/editor/footprint/canvas/tests/armed_drag_track_end_wins_walk_order_via_dispatcher.md) |
| called_by | [disarmed_select_tool_still_drags_whole_line_via_dispatcher](/crates/oxide-app/src/library/editor/footprint/canvas/tests/disarmed_select_tool_still_drags_whole_line_via_dispatcher.md) |
