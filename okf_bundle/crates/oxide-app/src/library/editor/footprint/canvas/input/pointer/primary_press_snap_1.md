---
okf_version: "0.2"
type: Function
title: primary_press_snap
description: "v0.18.8 / v0.27 — resolve the press-time world position: Select"
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/primary_press_snap_1
language: rust
---

# primary_press_snap

v0.18.8 / v0.27 — resolve the press-time world position: Select

## Signature

```rust
fn primary_press_snap(
        &self,
        cstate: &mut FootprintCanvasState,
        raw_world: (f64, f64),
    ) -> (f64, f64)
```

## Docstring

v0.18.8 / v0.27 — resolve the press-time world position: Select
tools use the raw cursor (so a click can target a specific
entity), placement tools go through `snap::snap_cursor`. Also
updates `cstate.last_snap` for the snap-kind badge.

## Source
Lines 175–204 in `crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pointer](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.md) |
| calls | [sketch_snap](/crates/oxide-app/src/library/editor/footprint/canvas/hit_test/sketch_snap.md) |
| calls | [snap_cursor](/crates/oxide-app/src/library/editor/footprint/snap/snap_cursor.md) |
