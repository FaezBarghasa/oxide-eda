---
okf_version: "0.2"
type: Function
title: advance_to
description: "Advances lockstep time to `target_time_s` and drains scheduled discrete events."
resource: crates/oxide-cosim/src/mixed_signal.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-cosim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:17:02Z"
concept_id: crates/oxide-cosim/src/mixed_signal/advance_to_1
language: rust
---

# advance_to

Advances lockstep time to `target_time_s` and drains scheduled discrete events.

## Signature

```rust
pub fn advance_to(&mut self, target_time_s: f64) -> Vec<LogicEvent>
```

## Visibility

- `pub`

## Docstring

Advances lockstep time to `target_time_s` and drains scheduled discrete events.

## Source
Lines 305–312 in `crates/oxide-cosim/src/mixed_signal.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mixed_signal](/crates/oxide-cosim/src/mixed_signal.md) |
