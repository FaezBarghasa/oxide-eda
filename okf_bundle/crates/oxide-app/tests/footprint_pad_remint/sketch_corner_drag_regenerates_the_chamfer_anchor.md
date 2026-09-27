---
okf_version: "0.2"
type: Function
title: sketch_corner_drag_regenerates_the_chamfer_anchor
description: "THE INVARIANT (h), the Sketch-mode corner drag — the edge drag's"
resource: crates/oxide-app/tests/footprint_pad_remint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_remint/sketch_corner_drag_regenerates_the_chamfer_anchor
language: rust
---

# sketch_corner_drag_regenerates_the_chamfer_anchor

THE INVARIANT (h), the Sketch-mode corner drag — the edge drag's

## Signature

```rust
fn sketch_corner_drag_regenerates_the_chamfer_anchor()
```

## Decorators

- `test`

## Docstring

THE INVARIANT (h), the Sketch-mode corner drag — the edge drag's
structurally identical sibling in the same file. It recomputes the
pad bbox from the four corner Points and rewrites `size_mm` /
`position_mm`, which is the same frame change, and it re-placed the
same four corners and nothing else.

Dragged in two ticks, as the pointer streams it, so this also guards
the drag continuity the edge-drag test guards: the corner Point's id
has to survive the first tick or the second addresses nothing.

Pulling the NE corner 0.5 mm up gives the same 2×1.5 mm pad at
(0, −0.25) the edge-drag test produces, so the NE chamfer anchor
lands in the same place: (0.625, −1.0).
[test]

## Source
Lines 630–668 in `crates/oxide-app/tests/footprint_pad_remint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_remint](/crates/oxide-app/tests/footprint_pad_remint.md) |
| calls | [editor_with_minted_pad](/crates/oxide-app/tests/footprint_pad_remint/editor_with_minted_pad.md) |
| calls | [chamfered_repro_pad](/crates/oxide-app/tests/footprint_pad_remint/chamfered_repro_pad.md) |
| calls | [dispatch](/crates/oxide-app/tests/footprint_pad_remint/dispatch.md) |
| calls | [sidecar_point](/crates/oxide-app/tests/footprint_pad_remint/sidecar_point.md) |
