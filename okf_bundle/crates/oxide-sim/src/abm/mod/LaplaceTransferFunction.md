---
okf_version: "0.2"
type: Class
title: LaplaceTransferFunction
description: "Rational s-domain Laplace Transfer Function: H(s) = N(s) / D(s)."
resource: crates/oxide-sim/src/abm/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:29:23Z"
concept_id: crates/oxide-sim/src/abm/mod/LaplaceTransferFunction
language: rust
---

# LaplaceTransferFunction

Rational s-domain Laplace Transfer Function: H(s) = N(s) / D(s).

## Signature

```rust
pub struct LaplaceTransferFunction
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Rational s-domain Laplace Transfer Function: H(s) = N(s) / D(s).
N(s) = sum(b_m * s^m), D(s) = sum(a_k * s^k).
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `numerator_coeffs`
- `denominator_coeffs`

## Source
Lines 13–16 in `crates/oxide-sim/src/abm/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [abm](/crates/oxide-sim/src/abm/mod.md) |
