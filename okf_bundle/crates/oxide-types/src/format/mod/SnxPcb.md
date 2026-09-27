---
okf_version: "0.2"
type: Class
title: SnxPcb
description: "On-disk representation of a `.snxpcb` file."
resource: crates/oxide-types/src/format/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/format/mod/SnxPcb
language: rust
---

# SnxPcb

On-disk representation of a `.snxpcb` file.

## Signature

```rust
pub struct SnxPcb
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

On-disk representation of a `.snxpcb` file.

Same shape as [`SnxSchematic`]: TOML manifest at the top, bulk
TSV blocks for footprints / pads / tracks / vias, regular TOML
for hierarchical or rare-field data (zone polygons, the stackup,
custom properties).
[derive(Debug, Clone)]

## Methods

- `format`
- `board`

## Source
Lines 401–404 in `crates/oxide-types/src/format/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [format](/crates/oxide-types/src/format/mod.md) |
