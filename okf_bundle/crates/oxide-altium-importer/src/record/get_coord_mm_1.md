---
okf_version: "0.2"
type: Function
title: get_coord_mm
description: Convert Altium DXP internal coordinate (1/10000 inch = 0.1 mil = 0.00254 mm) or point to mm.
resource: crates/oxide-altium-importer/src/record.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-altium-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:45:59Z"
concept_id: crates/oxide-altium-importer/src/record/get_coord_mm_1
language: rust
---

# get_coord_mm

Convert Altium DXP internal coordinate (1/10000 inch = 0.1 mil = 0.00254 mm) or point to mm.

## Signature

```rust
pub fn get_coord_mm(&self, key: &str) -> Option<f64>
```

## Visibility

- `pub`

## Docstring

Convert Altium DXP internal coordinate (1/10000 inch = 0.1 mil = 0.00254 mm) or point to mm.

## Source
Lines 42–46 in `crates/oxide-altium-importer/src/record.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [record](/crates/oxide-altium-importer/src/record.md) |
