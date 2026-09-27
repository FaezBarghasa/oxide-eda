---
okf_version: "0.2"
type: Function
title: should_place_via
description: Predict optimal layer transition / via placement
resource: crates/oxide-ml/src/via_planner.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ml"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T10:17:00Z"
concept_id: crates/oxide-ml/src/via_planner/should_place_via_1
language: rust
---

# should_place_via

Predict optimal layer transition / via placement

## Signature

```rust
pub fn should_place_via(
        &self,
        current_pos: Point2D,
        current_layer: u32,
        target_layer: u32,
        blocked_ahead: bool,
    ) -> ViaPrediction
```

## Visibility

- `pub`

## Docstring

Predict optimal layer transition / via placement

## Source
Lines 24–54 in `crates/oxide-ml/src/via_planner.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [via_planner](/crates/oxide-ml/src/via_planner.md) |
