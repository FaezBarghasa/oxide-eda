---
okf_version: "0.2"
type: Function
title: sample_voltage
description: "Evaluates the continuous instantaneous output voltage at time `t`."
resource: crates/oxide-cosim/src/mixed_signal.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-cosim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:17:02Z"
concept_id: crates/oxide-cosim/src/mixed_signal/sample_voltage
language: rust
---

# sample_voltage

Evaluates the continuous instantaneous output voltage at time `t`.

## Signature

```rust
impl DtoAGateway { pub fn sample_voltage(&self, t: f64) -> f64 }
```

## Visibility

- `pub`

## Docstring

Evaluates the continuous instantaneous output voltage at time `t`.

## Source
Lines 262–276 in `crates/oxide-cosim/src/mixed_signal.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mixed_signal](/crates/oxide-cosim/src/mixed_signal.md) |
