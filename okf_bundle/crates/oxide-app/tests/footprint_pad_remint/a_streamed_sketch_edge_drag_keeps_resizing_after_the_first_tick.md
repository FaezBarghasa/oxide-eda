---
okf_version: "0.2"
type: Function
title: a_streamed_sketch_edge_drag_keeps_resizing_after_the_first_tick
description: "The other half of (g), and the reason the fix cannot be a plain"
resource: crates/oxide-app/tests/footprint_pad_remint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_remint/a_streamed_sketch_edge_drag_keeps_resizing_after_the_first_tick
language: rust
---

# a_streamed_sketch_edge_drag_keeps_resizing_after_the_first_tick

The other half of (g), and the reason the fix cannot be a plain

## Signature

```rust
fn a_streamed_sketch_edge_drag_keeps_resizing_after_the_first_tick()
```

## Decorators

- `test`

## Docstring

The other half of (g), and the reason the fix cannot be a plain
drop-and-re-mint. A live edge drag streams one `SketchMoveLine` per
cursor tick, all carrying the id the pointer latched at press
(`drag_tick_line`, `canvas/input/pointer.rs`). Re-minting replaces
the outline with fresh UUIDs, so the second tick would address a
`Line` that no longer exists and the drag would freeze after the
first frame — the pad stuck one tick wide while the cursor keeps
moving.

Two ticks of 0.25 mm must resize exactly as far as one of 0.5 mm.
[test]

## Source
Lines 591–614 in `crates/oxide-app/tests/footprint_pad_remint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_remint](/crates/oxide-app/tests/footprint_pad_remint.md) |
| calls | [editor_with_minted_pad](/crates/oxide-app/tests/footprint_pad_remint/editor_with_minted_pad.md) |
| calls | [chamfered_repro_pad](/crates/oxide-app/tests/footprint_pad_remint/chamfered_repro_pad.md) |
| calls | [horizontal_edge_at_y](/crates/oxide-app/tests/footprint_pad_remint/horizontal_edge_at_y.md) |
| calls | [dispatch](/crates/oxide-app/tests/footprint_pad_remint/dispatch.md) |
