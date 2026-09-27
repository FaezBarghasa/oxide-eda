---
okf_version: "0.2"
type: Class
title: DimensionKind
description: Associative Dimensioning Types.
resource: crates/oxide-output/src/draftsman/gdt.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:19:20Z"
concept_id: crates/oxide-output/src/draftsman/gdt/DimensionKind
language: rust
---

# DimensionKind

Associative Dimensioning Types.

## Signature

```rust
pub enum DimensionKind
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Associative Dimensioning Types.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `p1`
- `p2`
- `text_pos`
- `measured_value_mm`
- `tolerance_plus_mm`
- `tolerance_minus_mm`
- `datum_origin`
- `target_point`
- `is_horizontal`
- `measured_value_mm`
- `center`
- `rim_point`
- `radius_mm`
- `center`
- `diameter_mm`
- `anchor_point`
- `frame`

## Source
Lines 120–148 in `crates/oxide-output/src/draftsman/gdt.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [gdt](/crates/oxide-output/src/draftsman/gdt.md) |
