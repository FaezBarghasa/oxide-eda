---
okf_version: "0.2"
type: Function
title: arc_sweep_exceeds_full_turn
description: "`true` when the Place Arc gesture's raw, unwrapped drag delta"
resource: crates/oxide-app/src/library/editor/symbol/canvas/input/tools.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/input/tools/arc_sweep_exceeds_full_turn
language: rust
---

# arc_sweep_exceeds_full_turn

`true` when the Place Arc gesture's raw, unwrapped drag delta

## Signature

```rust
fn arc_sweep_exceeds_full_turn(start_deg: f64, end_deg: f64) -> bool
```

## Docstring

`true` when the Place Arc gesture's raw, unwrapped drag delta
(`end_deg - start_deg`, BEFORE `normalize_arc_commit_deg`'s
swap-and-`rem_euclid`) covers a full turn or more. Committing such
a drag would collapse to `start_deg == end_deg` after
normalization — a zero-sweep point-arc that's invisible,
unselectable, and un-deletable via the canvas, yet still saves to
disk and still occupies its bounding box.

## Source
Lines 324–326 in `crates/oxide-app/src/library/editor/symbol/canvas/input/tools.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tools](/crates/oxide-app/src/library/editor/symbol/canvas/input/tools.md) |
| called_by | [on_left_press](/crates/oxide-app/src/library/editor/symbol/canvas/input/tools/on_left_press.md) |
