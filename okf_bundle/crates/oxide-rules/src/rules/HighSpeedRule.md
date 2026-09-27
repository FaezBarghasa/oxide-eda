---
okf_version: "0.2"
type: Class
title: HighSpeedRule
description: "High-speed differential pair, impedance, and length matching constraints."
resource: crates/oxide-rules/src/rules.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:26:29Z"
concept_id: crates/oxide-rules/src/rules/HighSpeedRule
language: rust
---

# HighSpeedRule

High-speed differential pair, impedance, and length matching constraints.

## Signature

```rust
pub struct HighSpeedRule
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

High-speed differential pair, impedance, and length matching constraints.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `net_class`
- `impedance_target`
- `length_tolerance`
- `max_uncoupled_length`

## Source
Lines 72–80 in `crates/oxide-rules/src/rules.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rules](/crates/oxide-rules/src/rules.md) |
