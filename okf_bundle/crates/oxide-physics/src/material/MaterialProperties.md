---
okf_version: "0.2"
type: Class
title: MaterialProperties
description: Physical and electrical characteristics of a PCB substrate or conductor material.
resource: crates/oxide-physics/src/material.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:09:18Z"
concept_id: crates/oxide-physics/src/material/MaterialProperties
language: rust
---

# MaterialProperties

Physical and electrical characteristics of a PCB substrate or conductor material.

## Signature

```rust
pub struct MaterialProperties
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Physical and electrical characteristics of a PCB substrate or conductor material.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `name`
- `dielectric_constant`
- `loss_tangent`
- `thermal_conductivity`

## Source
Lines 7–16 in `crates/oxide-physics/src/material.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [material](/crates/oxide-physics/src/material.md) |
