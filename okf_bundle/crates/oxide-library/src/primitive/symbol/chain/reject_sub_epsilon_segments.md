---
okf_version: "0.2"
type: Function
title: reject_sub_epsilon_segments
description: "Reject any segment whose total tessellated *length* is shorter than"
resource: crates/oxide-library/src/primitive/symbol/chain.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/chain/reject_sub_epsilon_segments
language: rust
---

# reject_sub_epsilon_segments

Reject any segment whose total tessellated *length* is shorter than

## Signature

```rust
fn reject_sub_epsilon_segments(polylines: &[Vec<[f64; 2]>]) -> Result<(), ChainError>
```

## Docstring

Reject any segment whose total tessellated *length* is shorter than
[`CHAIN_ENDPOINT_EPSILON_MM`], as [`ChainError::DegenerateSegment`].

Length here is [`polyline_length`] — the sum of consecutive
tessellated-point distances — **not** the chord (straight-line
distance) between the segment's two endpoints. For a `Line` (a
2-point polyline) the two are identical, but for an `Arc` they
diverge badly near a 360° sweep: `Arc { radius: 2.0, start_deg: 0.0,
end_deg: 359.9 }`'s two endpoints sit only `2r·sin(0.05°) ≈ 0.0035
mm` apart (its chord), while its true length is `≈ 12.56 mm` — almost
a full circumference. Gating on the chord would reject that arc
outright even though it's real, hit-test-selectable geometry; gating
on length lets it through to normal endpoint clustering instead,
where its own tiny chord gap then self-closes it into a ring (see
the `near_full_sweep_arc_*` test in `chain_tests.rs`).

This still catches both a near-zero-length `Line` stub and a
zero-sweep `Arc` — its tessellated points are all the same point, so
its length is exactly `0.0` (see the module doc comment). Left
unchecked, such a stub's two coincident ends land on whatever node
its real endpoint touches, inflating that node's degree and faking a
[`ChainError::Branching`] at an otherwise clean corner.

## Source
Lines 231–238 in `crates/oxide-library/src/primitive/symbol/chain.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [chain](/crates/oxide-library/src/primitive/symbol/chain.md) |
| calls | [polyline_length](/crates/oxide-library/src/primitive/symbol/chain/polyline_length.md) |
| called_by | [chain_into_closed_contour](/crates/oxide-library/src/primitive/symbol/chain/chain_into_closed_contour.md) |
