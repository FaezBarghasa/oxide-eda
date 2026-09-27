---
okf_version: "0.2"
type: Function
title: snap_value_to_grid
resource: crates/oxide-app/src/library/editor/symbol/state/movement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/movement/snap_value_to_grid
language: rust
---

# snap_value_to_grid

## Signature

```rust
fn snap_value_to_grid(v: f64, step: f64) -> f64
```

## Source
Lines 177–179 in `crates/oxide-app/src/library/editor/symbol/state/movement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [movement](/crates/oxide-app/src/library/editor/symbol/state/movement.md) |
| called_by | [snap_graphic_to_grid](/crates/oxide-app/src/library/editor/symbol/state/movement/snap_graphic_to_grid.md) |
| called_by | [snap_pin_to_grid](/crates/oxide-app/src/library/editor/symbol/state/movement/snap_pin_to_grid.md) |
