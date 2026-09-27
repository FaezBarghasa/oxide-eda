---
okf_version: "0.2"
type: Function
title: step_transient
description: Advances transient thermal diffusion by timestep $\Delta t$ (in seconds).
resource: crates/oxide-compute/src/thermal_grid.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:42:15Z"
concept_id: crates/oxide-compute/src/thermal_grid/step_transient_1
language: rust
---

# step_transient

Advances transient thermal diffusion by timestep $\Delta t$ (in seconds).

## Signature

```rust
pub fn step_transient(&mut self, dt_sec: f64)
```

## Visibility

- `pub`

## Docstring

Advances transient thermal diffusion by timestep $\Delta t$ (in seconds).

## Source
Lines 106–147 in `crates/oxide-compute/src/thermal_grid.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [thermal_grid](/crates/oxide-compute/src/thermal_grid.md) |
