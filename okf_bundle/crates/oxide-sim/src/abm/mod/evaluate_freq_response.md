---
okf_version: "0.2"
type: Function
title: evaluate_freq_response
description: "Evaluates frequency response H(j*omega) at given frequency in Hz."
resource: crates/oxide-sim/src/abm/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:29:23Z"
concept_id: crates/oxide-sim/src/abm/mod/evaluate_freq_response
language: rust
---

# evaluate_freq_response

Evaluates frequency response H(j*omega) at given frequency in Hz.

## Signature

```rust
impl LaplaceTransferFunction { pub fn evaluate_freq_response(&self, freq_hz: f64) -> (f64, f64) }
```

## Visibility

- `pub`

## Docstring

Evaluates frequency response H(j*omega) at given frequency in Hz.

## Source
Lines 27–66 in `crates/oxide-sim/src/abm/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [abm](/crates/oxide-sim/src/abm/mod.md) |
