---
okf_version: "0.2"
type: Function
title: evaluate_corridor
resource: crates/oxide-router/src/interactive/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:13:12Z"
concept_id: crates/oxide-router/src/interactive/mod/evaluate_corridor
language: rust
---

# evaluate_corridor

## Signature

```rust
impl InteractiveRouter { fn evaluate_corridor(&mut self, start: Point2D, cursor: Point2D, net_id: u32) -> Result<Vec<Point2D>, RoutingError> }
```

## Source
Lines 545–554 in `crates/oxide-router/src/interactive/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [interactive](/crates/oxide-router/src/interactive/mod.md) |
