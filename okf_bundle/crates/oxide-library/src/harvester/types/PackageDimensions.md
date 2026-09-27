---
okf_version: "0.2"
type: Class
title: PackageDimensions
description: Mechanical package dimensions extracted or synthesized for IPC-7351C footprint generation.
resource: crates/oxide-library/src/harvester/types.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:54:15Z"
concept_id: crates/oxide-library/src/harvester/types/PackageDimensions
language: rust
---

# PackageDimensions

Mechanical package dimensions extracted or synthesized for IPC-7351C footprint generation.

## Signature

```rust
pub struct PackageDimensions
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Mechanical package dimensions extracted or synthesized for IPC-7351C footprint generation.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `package_class`
- `pin_count`
- `body_length_mm`
- `body_width_mm`
- `seated_height_mm`
- `lead_pitch_mm`
- `lead_width_mm`
- `lead_length_mm`
- `thermal_pad_mm`

## Source
Lines 189–200 in `crates/oxide-library/src/harvester/types.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [types](/crates/oxide-library/src/harvester/types.md) |
