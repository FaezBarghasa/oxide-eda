---
okf_version: "0.2"
type: Function
title: solve_step
resource: crates/oxide-sim/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:33:35Z"
concept_id: crates/oxide-sim/src/engine/solve_step_1
language: rust
---

# solve_step

## Signature

```rust
fn solve_step(&mut self, _current_time: f64, target_dt: f64) -> Result<StepTelemetry, SimError>
```

## Source
Lines 267–323 in `crates/oxide-sim/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-sim/src/engine.md) |
