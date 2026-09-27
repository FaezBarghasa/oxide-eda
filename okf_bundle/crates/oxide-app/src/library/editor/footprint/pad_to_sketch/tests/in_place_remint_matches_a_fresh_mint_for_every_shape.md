---
okf_version: "0.2"
type: Function
title: in_place_remint_matches_a_fresh_mint_for_every_shape
description: "#434 — every existing in-place-remint assertion above used a"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/in_place_remint_matches_a_fresh_mint_for_every_shape
language: rust
---

# in_place_remint_matches_a_fresh_mint_for_every_shape

#434 — every existing in-place-remint assertion above used a

## Signature

```rust
fn in_place_remint_matches_a_fresh_mint_for_every_shape()
```

## Decorators

- `test`

## Docstring

#434 — every existing in-place-remint assertion above used a
Chamfered pad, whose anchors are named directly on `shape_params`
so `pair_sidecar_entities` finds them in one step. RoundRect's 4
inset arc centres are the one sidecar geometry reachable only by
descending through its 4 `Arc` entities' `center` field (the Arc
arm in `pair_sidecar_entities`, remint_in_place.rs ~167-184). Oval,
by contrast, seeds its 2 arc centres (`oval_centre_0`/
`oval_centre_1`) as direct sidecars exactly like Chamfered — it
exercises no Arc-descent path here, and stays in this walk only for
its own from-scratch-equality coverage. Parameterised over all four
pad shapes so the next shape added to this walk has to earn the
same coverage; the RoundRect case additionally asserts that the
PRE-REMINT ids themselves survive, since a from-scratch-equality
check alone stays green even when the pairing silently falls back
to a full re-mint under fresh ids.
[test]

## Source
Lines 982–999 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.md) |
| calls | [assert_in_place_remint_matches_fresh_mint](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/assert_in_place_remint_matches_fresh_mint.md) |
