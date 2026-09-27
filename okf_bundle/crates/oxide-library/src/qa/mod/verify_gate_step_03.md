---
okf_version: "0.2"
type: Function
title: verify_gate_step_03
description: "Validates GATE-STEP-03: 3D body sanity and coplanarity."
resource: crates/oxide-library/src/qa/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:57:45Z"
concept_id: crates/oxide-library/src/qa/mod/verify_gate_step_03
language: rust
---

# verify_gate_step_03

Validates GATE-STEP-03: 3D body sanity and coplanarity.

## Signature

```rust
impl VerificationEngine { pub fn verify_gate_step_03(footprint: &Footprint) -> Result<(), GateVerificationError> }
```

## Visibility

- `pub`

## Docstring

Validates GATE-STEP-03: 3D body sanity and coplanarity.

## Source
Lines 114–123 in `crates/oxide-library/src/qa/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [qa](/crates/oxide-library/src/qa/mod.md) |
