---
okf_version: "0.2"
type: Function
title: standard_qfn
description: Creates standard QFN dimensions.
resource: crates/oxide-library/src/harvester/types.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:54:15Z"
concept_id: crates/oxide-library/src/harvester/types/standard_qfn
language: rust
---

# standard_qfn

Creates standard QFN dimensions.

## Signature

```rust
impl PackageDimensions { pub fn standard_qfn(pin_count: usize, body_size_mm: f64, pitch_mm: f64, thermal_pad_mm: Option<[f64; 2]>) -> Self }
```

## Visibility

- `pub`

## Docstring

Creates standard QFN dimensions.

## Source
Lines 237–249 in `crates/oxide-library/src/harvester/types.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [types](/crates/oxide-library/src/harvester/types.md) |
