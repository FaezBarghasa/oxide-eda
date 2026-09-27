---
okf_version: "0.2"
type: Function
title: predict
description: Predict action scores for a given routing state
resource: crates/oxide-ml/src/routing_advisor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ml"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-18T20:00:48Z"
concept_id: crates/oxide-ml/src/routing_advisor/predict_1
language: rust
---

# predict

Predict action scores for a given routing state

## Signature

```rust
pub fn predict(
        &mut self,
        current_pos: Point2D,
        current_layer: u32,
        target_pos: Point2D,
        net_id: u32,
        obstacles: &[(Point2D, u32)],
    ) -> RoutingAdvisorOutput
```

## Visibility

- `pub`

## Docstring

Predict action scores for a given routing state

## Source
Lines 30–120 in `crates/oxide-ml/src/routing_advisor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [routing_advisor](/crates/oxide-ml/src/routing_advisor.md) |
