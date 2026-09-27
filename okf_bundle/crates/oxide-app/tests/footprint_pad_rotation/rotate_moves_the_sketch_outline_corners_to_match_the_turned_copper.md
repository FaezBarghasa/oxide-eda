---
okf_version: "0.2"
type: Function
title: rotate_moves_the_sketch_outline_corners_to_match_the_turned_copper
description: "The rotate arm mutated `rotation_deg` and called only"
resource: crates/oxide-app/tests/footprint_pad_rotation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_rotation/rotate_moves_the_sketch_outline_corners_to_match_the_turned_copper
language: rust
---

# rotate_moves_the_sketch_outline_corners_to_match_the_turned_copper

The rotate arm mutated `rotation_deg` and called only

## Signature

```rust
fn rotate_moves_the_sketch_outline_corners_to_match_the_turned_copper()
```

## Decorators

- `test`

## Docstring

The rotate arm mutated `rotation_deg` and called only
`sync_pads_to_primitive`, which writes `fp.pads` + the attribute
mirror and never repositions `corner_entity_ids`. The corner
`Point`s are moved by `mirror_move_pad_in_sketch` alone, which the
two structurally identical align arms in the same file DO call.

Result: copper rendered at 90° while the sketch construction outline
still showed the 0° corners — the exact "derived geometry no longer
matches the copper" failure this branch exists to fix, reintroduced
by the very button issue #390 is about.

This drives the real `ActiveBarRotateSelection` message and asserts
the sketch `Point` positions; the mint-time closure invariants
cannot see this because they only ever run at mint time.
[test]

## Source
Lines 354–367 in `crates/oxide-app/tests/footprint_pad_rotation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_rotation](/crates/oxide-app/tests/footprint_pad_rotation.md) |
| calls | [sketched_pad_fixture](/crates/oxide-app/tests/footprint_pad_rotation/sketched_pad_fixture.md) |
| calls | [dispatch](/crates/oxide-app/tests/footprint_pad_rotation/dispatch.md) |
| calls | [assert_corners_match_pad](/crates/oxide-app/tests/footprint_pad_rotation/assert_corners_match_pad.md) |
