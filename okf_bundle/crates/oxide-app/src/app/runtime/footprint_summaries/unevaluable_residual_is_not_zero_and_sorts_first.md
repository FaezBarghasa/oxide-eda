---
okf_version: "0.2"
type: Function
title: unevaluable_residual_is_not_zero_and_sorts_first
description: "GH #599 — a residual that cannot be evaluated is reported as"
resource: crates/oxide-app/src/app/runtime/footprint_summaries.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/runtime/footprint_summaries/unevaluable_residual_is_not_zero_and_sorts_first
language: rust
---

# unevaluable_residual_is_not_zero_and_sorts_first

GH #599 — a residual that cannot be evaluated is reported as

## Signature

```rust
fn unevaluable_residual_is_not_zero_and_sorts_first()
```

## Decorators

- `test`

## Docstring

GH #599 — a residual that cannot be evaluated is reported as
unavailable and sorts FIRST. It used to be substituted with
0.0, which the descending sort then pushed below constraints
with tiny nonzero residuals, steering the user away from the
conflict they opened the panel to find.
[test]

## Source
Lines 331–363 in `crates/oxide-app/src/app/runtime/footprint_summaries.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_summaries](/crates/oxide-app/src/app/runtime/footprint_summaries.md) |
| calls | [two_point_scaffold](/crates/oxide-app/src/app/runtime/footprint_summaries/two_point_scaffold.md) |
| calls | [distance](/crates/oxide-app/src/app/runtime/footprint_summaries/distance.md) |
| calls | [output_for](/crates/oxide-app/src/app/runtime/footprint_summaries/output_for.md) |
| calls | [build_over_constraint_summaries](/crates/oxide-app/src/app/runtime/footprint_summaries/build_over_constraint_summaries.md) |
