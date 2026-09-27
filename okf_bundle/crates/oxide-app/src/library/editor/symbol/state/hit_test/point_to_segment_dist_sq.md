---
okf_version: "0.2"
type: Function
title: point_to_segment_dist_sq
resource: crates/oxide-app/src/library/editor/symbol/state/hit_test.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/hit_test/point_to_segment_dist_sq
language: rust
---

# point_to_segment_dist_sq

## Signature

```rust
fn point_to_segment_dist_sq(p: [f64; 2], a: [f64; 2], b: [f64; 2]) -> f64
```

## Source
Lines 172–174 in `crates/oxide-app/src/library/editor/symbol/state/hit_test.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hit_test](/crates/oxide-app/src/library/editor/symbol/state/hit_test.md) |
| calls | [point_to_segment_distance_sq](/crates/oxide-sketch/src/geom/point/point_to_segment_distance_sq.md) |
| called_by | [hit_test_graphic_body](/crates/oxide-app/src/library/editor/symbol/state/hit_test/hit_test_graphic_body.md) |
| called_by | [polygon_outline_hit](/crates/oxide-app/src/library/editor/symbol/state/hit_test/polygon_outline_hit.md) |
