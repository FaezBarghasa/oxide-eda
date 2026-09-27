---
okf_version: "0.2"
type: Function
title: local_to_world_mm
description: "Map a POINT given in the pad's own frame — the frame"
resource: crates/oxide-app/src/library/editor/footprint/state/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/pad/local_to_world_mm
language: rust
---

# local_to_world_mm

Map a POINT given in the pad's own frame — the frame

## Signature

```rust
impl EditorPad { pub fn local_to_world_mm(&self, x: f64, y: f64) -> (f64, f64) }
```

## Visibility

- `pub`

## Docstring

Map a POINT given in the pad's own frame — the frame
[`Self::bbox_mm`] is expressed in — into world mm.

Every position derived off `bbox_mm` (round-rect arc anchors,
chamfer anchors, oval arc centres, the resized-edge corner
targets) has to come back through here, or the derived geometry
stays axis-aligned while the corners it is supposed to join turn
with the pad, and the outline no longer closes.

## Source
Lines 225–229 in `crates/oxide-app/src/library/editor/footprint/state/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/library/editor/footprint/state/pad.md) |
