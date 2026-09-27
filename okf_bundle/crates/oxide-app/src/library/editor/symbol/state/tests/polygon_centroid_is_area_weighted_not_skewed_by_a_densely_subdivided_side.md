---
okf_version: "0.2"
type: Function
title: polygon_centroid_is_area_weighted_not_skewed_by_a_densely_subdivided_side
description: A rectangle-as-polygon whose top side is densely subdivided into
resource: crates/oxide-app/src/library/editor/symbol/state/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/tests/polygon_centroid_is_area_weighted_not_skewed_by_a_densely_subdivided_side
language: rust
---

# polygon_centroid_is_area_weighted_not_skewed_by_a_densely_subdivided_side

A rectangle-as-polygon whose top side is densely subdivided into

## Signature

```rust
fn polygon_centroid_is_area_weighted_not_skewed_by_a_densely_subdivided_side()
```

## Decorators

- `test`

## Docstring

A rectangle-as-polygon whose top side is densely subdivided into
many extra collinear points (mimicking a tessellated arc side from
a Join-into-Polygon result) still centres at the true geometric
centre — a plain vertex mean would skew toward the densely
subdivided side instead.
[test]

## Source
Lines 595–610 in `crates/oxide-app/src/library/editor/symbol/state/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/symbol/state/tests.md) |
| calls | [polygon_centroid](/crates/oxide-app/src/library/editor/symbol/state/mod/polygon_centroid.md) |
