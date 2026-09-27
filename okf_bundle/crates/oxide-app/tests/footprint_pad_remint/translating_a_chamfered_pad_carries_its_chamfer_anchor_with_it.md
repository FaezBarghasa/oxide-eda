---
okf_version: "0.2"
type: Function
title: translating_a_chamfered_pad_carries_its_chamfer_anchor_with_it
description: "THE INVARIANT (f), the TRANSLATION siblings — pad drag, nudge,"
resource: crates/oxide-app/tests/footprint_pad_remint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_remint/translating_a_chamfered_pad_carries_its_chamfer_anchor_with_it
language: rust
---

# translating_a_chamfered_pad_carries_its_chamfer_anchor_with_it

THE INVARIANT (f), the TRANSLATION siblings — pad drag, nudge,

## Signature

```rust
fn translating_a_chamfered_pad_carries_its_chamfer_anchor_with_it()
```

## Decorators

- `test`

## Docstring

THE INVARIANT (f), the TRANSLATION siblings — pad drag, nudge,
Move-By, align-to-grid, move-origin-to-grid, align/distribute. Six
call sites, all routing through `mirror_move_pad_in_sketch`, and
the pad frame's ORIGIN is as much a part of the frame as its angle:
every anchor is placed through `local_to_world_mm`, which is
centred on `position_mm`.

The corner mover moved the centre and the four bbox corners and
nothing else, so a dragged Chamfered pad left its chamfer anchors
behind at the old location entirely. This one is fixed by
translating the sidecar rather than re-minting it: a translation
moves every owned point by the same delta, and a re-mint on every
drag frame would destroy the user's constraints for nothing.
[test]

## Source
Lines 451–479 in `crates/oxide-app/tests/footprint_pad_remint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_remint](/crates/oxide-app/tests/footprint_pad_remint.md) |
| calls | [editor_with_minted_pad](/crates/oxide-app/tests/footprint_pad_remint/editor_with_minted_pad.md) |
| calls | [chamfered_repro_pad](/crates/oxide-app/tests/footprint_pad_remint/chamfered_repro_pad.md) |
| calls | [dispatch](/crates/oxide-app/tests/footprint_pad_remint/dispatch.md) |
| calls | [sidecar_point](/crates/oxide-app/tests/footprint_pad_remint/sidecar_point.md) |
