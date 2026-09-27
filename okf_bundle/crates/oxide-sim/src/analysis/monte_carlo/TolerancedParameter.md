---
okf_version: "0.2"
type: Class
title: TolerancedParameter
description: A component parameter subject to statistical Monte Carlo variations.
resource: crates/oxide-sim/src/analysis/monte_carlo.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:30:03Z"
concept_id: crates/oxide-sim/src/analysis/monte_carlo/TolerancedParameter
language: rust
---

# TolerancedParameter

A component parameter subject to statistical Monte Carlo variations.

## Signature

```rust
pub struct TolerancedParameter
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

A component parameter subject to statistical Monte Carlo variations.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `name`
- `nominal_value`
- `distribution`

## Source
Lines 19–23 in `crates/oxide-sim/src/analysis/monte_carlo.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [monte_carlo](/crates/oxide-sim/src/analysis/monte_carlo.md) |
