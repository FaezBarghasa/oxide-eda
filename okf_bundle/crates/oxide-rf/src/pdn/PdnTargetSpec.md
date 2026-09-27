---
okf_version: "0.2"
type: Class
title: PdnTargetSpec
description: Target Impedance Specification for a Power Rail.
resource: crates/oxide-rf/src/pdn.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-rf"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:41:24Z"
concept_id: crates/oxide-rf/src/pdn/PdnTargetSpec
language: rust
---

# PdnTargetSpec

Target Impedance Specification for a Power Rail.

## Signature

```rust
pub struct PdnTargetSpec
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Target Impedance Specification for a Power Rail.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `rail_name`
- `nominal_voltage_v`
- `allowed_ripple_fraction`
- `transient_current_a`
- `f_max_hz`

## Source
Lines 12–18 in `crates/oxide-rf/src/pdn.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pdn](/crates/oxide-rf/src/pdn.md) |
