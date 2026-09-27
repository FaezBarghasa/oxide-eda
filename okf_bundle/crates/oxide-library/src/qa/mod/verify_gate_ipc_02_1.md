---
okf_version: "0.2"
type: Function
title: verify_gate_ipc_02
description: "Validates GATE-IPC-02: Minimum copper clearance >= 0.10 mm."
resource: crates/oxide-library/src/qa/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:57:45Z"
concept_id: crates/oxide-library/src/qa/mod/verify_gate_ipc_02_1
language: rust
---

# verify_gate_ipc_02

Validates GATE-IPC-02: Minimum copper clearance >= 0.10 mm.

## Signature

```rust
pub fn verify_gate_ipc_02(footprint: &Footprint, min_clearance_mm: f64) -> Result<(), GateVerificationError>
```

## Visibility

- `pub`

## Docstring

Validates GATE-IPC-02: Minimum copper clearance >= 0.10 mm.

## Source
Lines 73–111 in `crates/oxide-library/src/qa/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [qa](/crates/oxide-library/src/qa/mod.md) |
