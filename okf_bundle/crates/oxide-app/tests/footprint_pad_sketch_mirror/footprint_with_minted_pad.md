---
okf_version: "0.2"
type: Function
title: footprint_with_minted_pad
description: "Mint one pad of `shape` at `pos` into a fresh footprint and return"
resource: crates/oxide-app/tests/footprint_pad_sketch_mirror.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_sketch_mirror/footprint_with_minted_pad
language: rust
---

# footprint_with_minted_pad

Mint one pad of `shape` at `pos` into a fresh footprint and return

## Signature

```rust
fn footprint_with_minted_pad(shape: PadShape, pos: (f64, f64) -> Footprint
```

## Docstring

Mint one pad of `shape` at `pos` into a fresh footprint and return
the footprint with the editor state already synced onto it — i.e.
exactly what would be written to disk.

## Source
Lines 32–41 in `crates/oxide-app/tests/footprint_pad_sketch_mirror.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_sketch_mirror](/crates/oxide-app/tests/footprint_pad_sketch_mirror.md) |
| calls | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
| called_by | [issue142_delete_does_not_eat_user_geometry_sharing_an_anchor](/crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_delete_does_not_eat_user_geometry_sharing_an_anchor.md) |
| called_by | [issue142_owned_ledger_survives_a_real_serde_round_trip](/crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_owned_ledger_survives_a_real_serde_round_trip.md) |
| called_by | [issue142_reopened_pad_delete_removes_its_geometry](/crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_reopened_pad_delete_removes_its_geometry.md) |
| called_by | [issue142_reopened_pad_still_moves_its_whole_outline](/crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_reopened_pad_still_moves_its_whole_outline.md) |
