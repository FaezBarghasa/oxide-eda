---
okf_version: "0.2"
type: Class
title: FpCutout
description: "Board cutout — subtracts from the PCB outline (mounting hole, slot,"
resource: crates/oxide-library/src/primitive/footprint/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/primitive/footprint/mod/FpCutout
language: rust
---

# FpCutout

Board cutout — subtracts from the PCB outline (mounting hole, slot,

## Signature

```rust
pub struct FpCutout
```

## Decorators

- `derive(Clone, Debug, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Board cutout — subtracts from the PCB outline (mounting hole, slot,
edge cutout). Outline subtraction itself runs at PCB-export time.
[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]

## Methods

- `boundary`
- `edge_radius_mm`
- `through`

## Source
Lines 221–232 in `crates/oxide-library/src/primitive/footprint/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint](/crates/oxide-library/src/primitive/footprint/mod.md) |
