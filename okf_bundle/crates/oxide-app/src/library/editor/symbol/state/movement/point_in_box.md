---
okf_version: "0.2"
type: Function
title: point_in_box
resource: crates/oxide-app/src/library/editor/symbol/state/movement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/movement/point_in_box
language: rust
---

# point_in_box

## Signature

```rust
fn point_in_box(x: f64, y: f64, xmin: f64, xmax: f64, ymin: f64, ymax: f64) -> bool
```

## Source
Lines 271–273 in `crates/oxide-app/src/library/editor/symbol/state/movement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [movement](/crates/oxide-app/src/library/editor/symbol/state/movement.md) |
| called_by | [segment_crosses_box](/crates/oxide-app/src/library/editor/symbol/state/movement/segment_crosses_box.md) |
| called_by | [select_in_box](/crates/oxide-app/src/library/editor/symbol/state/movement/select_in_box.md) |
