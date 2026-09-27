---
okf_version: "0.2"
type: Function
title: update_state
description: Schedules a discrete logic state update and initiates an exponential transition ramp.
resource: crates/oxide-cosim/src/mixed_signal.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-cosim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:17:02Z"
concept_id: crates/oxide-cosim/src/mixed_signal/update_state_1
language: rust
---

# update_state

Schedules a discrete logic state update and initiates an exponential transition ramp.

## Signature

```rust
pub fn update_state(&mut self, state: Logic12State, current_time_s: f64, current_v: f64)
```

## Visibility

- `pub`

## Docstring

Schedules a discrete logic state update and initiates an exponential transition ramp.

## Source
Lines 251–259 in `crates/oxide-cosim/src/mixed_signal.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mixed_signal](/crates/oxide-cosim/src/mixed_signal.md) |
