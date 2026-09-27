---
okf_version: "0.2"
type: Class
title: ThermalReliefStyle
description: Thermal-relief connection style for pads inside a pour. v0.14
resource: crates/oxide-library/src/primitive/footprint/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/primitive/footprint/mod/ThermalReliefStyle
language: rust
---

# ThermalReliefStyle

Thermal-relief connection style for pads inside a pour. v0.14

## Signature

```rust
pub enum ThermalReliefStyle
```

## Decorators

- `derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)`
- `serde(rename_all = "snake_case")`
- `non_exhaustive`

## Visibility

- `pub`

## Docstring

Thermal-relief connection style for pads inside a pour. v0.14
records the choice; the actual relief geometry is generated at
pour-render time (v0.15).
[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
[serde(rename_all = "snake_case")]
[non_exhaustive]

## Source
Lines 169–174 in `crates/oxide-library/src/primitive/footprint/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint](/crates/oxide-library/src/primitive/footprint/mod.md) |
