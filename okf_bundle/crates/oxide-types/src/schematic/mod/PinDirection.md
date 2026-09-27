---
okf_version: "0.2"
type: Class
title: PinDirection
description: Pin electrical role.
resource: crates/oxide-types/src/schematic/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:15:42Z"
concept_id: crates/oxide-types/src/schematic/mod/PinDirection
language: rust
---

# PinDirection

Pin electrical role.

## Signature

```rust
pub enum PinDirection
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)`
- `serde(rename_all = "snake_case")`

## Visibility

- `pub`

## Docstring

Pin electrical role.

Curated 14-variant set spanning generic digital pins, power pins,
open-drain polarity-tagged outputs, plus Oxide-original additions
(`GroundReference`, `Differential`, `Clock`) that don't appear in
other EDA tools' enums.
[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
[serde(rename_all = "snake_case")]

## Source
Lines 347–378 in `crates/oxide-types/src/schematic/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic](/crates/oxide-types/src/schematic/mod.md) |
