---
okf_version: "0.2"
type: Function
title: segment_crosses_box
resource: crates/oxide-app/src/library/editor/symbol/state/movement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/movement/segment_crosses_box
language: rust
---

# segment_crosses_box

## Signature

```rust
fn segment_crosses_box(
    a: [f64; 2],
    b: [f64; 2],
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
) -> bool
```

## Source
Lines 354–376 in `crates/oxide-app/src/library/editor/symbol/state/movement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [movement](/crates/oxide-app/src/library/editor/symbol/state/movement.md) |
| calls | [point_in_box](/crates/oxide-app/src/library/editor/symbol/state/movement/point_in_box.md) |
| calls | [segments_intersect](/crates/oxide-app/src/library/editor/symbol/state/movement/segments_intersect.md) |
| called_by | [select_in_box](/crates/oxide-app/src/library/editor/symbol/state/movement/select_in_box.md) |
