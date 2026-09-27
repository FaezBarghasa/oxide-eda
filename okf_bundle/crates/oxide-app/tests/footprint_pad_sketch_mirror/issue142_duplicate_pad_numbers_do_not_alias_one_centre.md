---
okf_version: "0.2"
type: Function
title: issue142_duplicate_pad_numbers_do_not_alias_one_centre
description: Two pads sharing a number must not share a sketch centre.
resource: crates/oxide-app/tests/footprint_pad_sketch_mirror.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_duplicate_pad_numbers_do_not_alias_one_centre
language: rust
---

# issue142_duplicate_pad_numbers_do_not_alias_one_centre

Two pads sharing a number must not share a sketch centre.

## Signature

```rust
fn issue142_duplicate_pad_numbers_do_not_alias_one_centre()
```

## Decorators

- `test`

## Docstring

Two pads sharing a number must not share a sketch centre.

Pad numbers are not unique anywhere in oxide — the designator field
takes any string, and `next_pad_defaults.designator_override` stamps
one number onto every pad placed after it, which is how a
shared-designator row / thermal / shield pad set is authored. Each
such pad mints its OWN `PadAttr`-bearing centre.

Relinking them by number alone is last-wins, so on reopen every pad
with that number got the SAME `sketch_entity_id` — and then a
Pads-mode delete of pad A ran the delete mirror over pad B's centre,
B's `PadAttr::owned` ledger and B's whole outline. B stayed in
`state.pads` looking healthy while the bake, which resolves copper
from the sketch, silently dropped its copper from the export.
[test]

## Source
Lines 271–288 in `crates/oxide-app/tests/footprint_pad_sketch_mirror.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_sketch_mirror](/crates/oxide-app/tests/footprint_pad_sketch_mirror.md) |
| calls | [footprint_with_two_pads_sharing_a_number](/crates/oxide-app/tests/footprint_pad_sketch_mirror/footprint_with_two_pads_sharing_a_number.md) |
