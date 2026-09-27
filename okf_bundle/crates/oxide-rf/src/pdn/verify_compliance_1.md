---
okf_version: "0.2"
type: Function
title: verify_compliance
description: "Verifies whether the PDN impedance profile satisfies $Z(f) \\le Z_{\\text{target}}$ across all test frequencies."
resource: crates/oxide-rf/src/pdn.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rf"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:41:24Z"
concept_id: crates/oxide-rf/src/pdn/verify_compliance_1
language: rust
---

# verify_compliance

Verifies whether the PDN impedance profile satisfies $Z(f) \le Z_{\text{target}}$ across all test frequencies.

## Signature

```rust
pub fn verify_compliance(
        z_profile: &[f64],
        z_target: f64,
    ) -> (bool, f64)
```

## Visibility

- `pub`

## Docstring

Verifies whether the PDN impedance profile satisfies $Z(f) \le Z_{\text{target}}$ across all test frequencies.

## Source
Lines 137–143 in `crates/oxide-rf/src/pdn.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pdn](/crates/oxide-rf/src/pdn.md) |
