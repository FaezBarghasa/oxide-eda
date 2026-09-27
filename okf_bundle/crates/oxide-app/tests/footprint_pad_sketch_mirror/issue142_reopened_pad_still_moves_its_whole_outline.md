---
okf_version: "0.2"
type: Function
title: issue142_reopened_pad_still_moves_its_whole_outline
description: A pad reopened from disk must still own its sketch geometry.
resource: crates/oxide-app/tests/footprint_pad_sketch_mirror.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_reopened_pad_still_moves_its_whole_outline
language: rust
---

# issue142_reopened_pad_still_moves_its_whole_outline

A pad reopened from disk must still own its sketch geometry.

## Signature

```rust
fn issue142_reopened_pad_still_moves_its_whole_outline()
```

## Decorators

- `test`

## Docstring

A pad reopened from disk must still own its sketch geometry.

This is the boundary the wave-1 tests never crossed. All three
editor-side ownership fields — `sketch_entity_id`,
`corner_entity_ids`, `shape_params` — are session-volatile:
`EditorPad::from_pad` sets every one of them to `None` / empty
because none has a home on `Pad`. So on reopen the pad owned
nothing AND had no link at all, and `mirror_move_pad_in_sketch`
early-returned on the missing link before ownership even mattered.
The pad's whole RoundRect outline stayed where it was minted while
the pad moved away, and the bake resolves copper from the sketch —
so the exported footprint had the copper in the old place.

Two fixes have to hold together for this to pass: the link is
rebuilt from the sketch's own `PadAttr.number`, and the owned set
is read from the durable `PadAttr::owned` ledger.
[test]

## Source
Lines 102–136 in `crates/oxide-app/tests/footprint_pad_sketch_mirror.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_sketch_mirror](/crates/oxide-app/tests/footprint_pad_sketch_mirror.md) |
| calls | [footprint_with_minted_pad](/crates/oxide-app/tests/footprint_pad_sketch_mirror/footprint_with_minted_pad.md) |
| calls | [points](/crates/oxide-app/tests/footprint_pad_sketch_mirror/points.md) |
| calls | [mirror_move_pad_in_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_move_pad_in_sketch.md) |
| calls | [point_xy](/crates/oxide-app/tests/footprint_pad_sketch_mirror/point_xy.md) |
