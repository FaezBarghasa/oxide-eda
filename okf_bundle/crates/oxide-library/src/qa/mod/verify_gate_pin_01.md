---
okf_version: "0.2"
type: Function
title: verify_gate_pin_01
description: "Validates GATE-PIN-01: Strict bijection between Symbol pins and Footprint pads."
resource: crates/oxide-library/src/qa/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:57:45Z"
concept_id: crates/oxide-library/src/qa/mod/verify_gate_pin_01
language: rust
---

# verify_gate_pin_01

Validates GATE-PIN-01: Strict bijection between Symbol pins and Footprint pads.

## Signature

```rust
impl VerificationEngine { pub fn verify_gate_pin_01(symbol: &Symbol, footprint: &Footprint) -> Result<(), GateVerificationError> }
```

## Visibility

- `pub`

## Docstring

Validates GATE-PIN-01: Strict bijection between Symbol pins and Footprint pads.

## Source
Lines 41–70 in `crates/oxide-library/src/qa/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [qa](/crates/oxide-library/src/qa/mod.md) |
