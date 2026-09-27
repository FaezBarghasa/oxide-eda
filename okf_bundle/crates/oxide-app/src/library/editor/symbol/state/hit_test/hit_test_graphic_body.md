---
okf_version: "0.2"
type: Function
title: hit_test_graphic_body
description: "Body hit test for the graphic at `idx`. Rectangle counts every"
resource: crates/oxide-app/src/library/editor/symbol/state/hit_test.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/hit_test/hit_test_graphic_body
language: rust
---

# hit_test_graphic_body

Body hit test for the graphic at `idx`. Rectangle counts every

## Signature

```rust
fn hit_test_graphic_body(sym: &Symbol, idx: usize, x: f64, y: f64) -> bool
```

## Docstring

Body hit test for the graphic at `idx`. Rectangle counts every
interior point; line / arc / circle count any point within the
stroke tolerance band so the user can grab thin strokes without
pixel-perfect aim.

## Source
Lines 94–170 in `crates/oxide-app/src/library/editor/symbol/state/hit_test.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hit_test](/crates/oxide-app/src/library/editor/symbol/state/hit_test.md) |
| calls | [point_to_segment_dist_sq](/crates/oxide-app/src/library/editor/symbol/state/hit_test/point_to_segment_dist_sq.md) |
| calls | [arc_is_full_turn_rad](/crates/oxide-gfx/src/primitive/arc/arc_is_full_turn_rad.md) |
| calls | [point_in_polygon](/crates/oxide-app/src/library/editor/symbol/state/hit_test/point_in_polygon.md) |
| calls | [polygon_outline_hit](/crates/oxide-app/src/library/editor/symbol/state/hit_test/polygon_outline_hit.md) |
| called_by | [hit_test](/crates/oxide-app/src/library/editor/symbol/state/hit_test/hit_test.md) |
