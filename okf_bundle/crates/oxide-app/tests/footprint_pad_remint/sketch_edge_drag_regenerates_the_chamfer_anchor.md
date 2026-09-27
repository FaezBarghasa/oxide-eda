---
okf_version: "0.2"
type: Function
title: sketch_edge_drag_regenerates_the_chamfer_anchor
description: "THE INVARIANT (g), the Sketch-mode edge drag. Dragging a pad edge"
resource: crates/oxide-app/tests/footprint_pad_remint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_remint/sketch_edge_drag_regenerates_the_chamfer_anchor
language: rust
---

# sketch_edge_drag_regenerates_the_chamfer_anchor

THE INVARIANT (g), the Sketch-mode edge drag. Dragging a pad edge

## Signature

```rust
fn sketch_edge_drag_regenerates_the_chamfer_anchor()
```

## Decorators

- `test`

## Docstring

THE INVARIANT (g), the Sketch-mode edge drag. Dragging a pad edge
changes both extents and centre — the same frame change the
Properties-panel Size field makes — but the handler regenerated only
the four bbox corners, through its `target_positions` catch-up loop.

The discriminating drag is the one that changes `min(w, h)`: a
Chamfered pad's chamfer length is `ratio × min(w, h)`, so widening
the SHORT axis makes the chamfer longer and the anchors move by
something other than the edge delta. A drag along the long axis
translates the anchors rigidly and happens to land them where a
fresh mint would — which is why this test drags the top edge, not
the right one.

2×1 mm dragged 0.5 mm up the top edge is 2×1.5 mm centred at
(0, −0.25); the chamfer length becomes 0.25 × 1.5 = 0.375, so the NE
anchor sits at pad-local (1 − 0.375, −0.75) = world (0.625, −1.0).
[test]

## Source
Lines 530–578 in `crates/oxide-app/tests/footprint_pad_remint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_remint](/crates/oxide-app/tests/footprint_pad_remint.md) |
| calls | [editor_with_minted_pad](/crates/oxide-app/tests/footprint_pad_remint/editor_with_minted_pad.md) |
| calls | [chamfered_repro_pad](/crates/oxide-app/tests/footprint_pad_remint/chamfered_repro_pad.md) |
| calls | [horizontal_edge_at_y](/crates/oxide-app/tests/footprint_pad_remint/horizontal_edge_at_y.md) |
| calls | [dispatch](/crates/oxide-app/tests/footprint_pad_remint/dispatch.md) |
| calls | [sidecar_point](/crates/oxide-app/tests/footprint_pad_remint/sidecar_point.md) |
| calls | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
