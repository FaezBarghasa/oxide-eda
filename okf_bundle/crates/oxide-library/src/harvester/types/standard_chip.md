---
okf_version: "0.2"
type: Function
title: standard_chip
description: "Creates standard 2-terminal SMD chip dimensions (e.g., 0402, 0603, 0805, 1206)."
resource: crates/oxide-library/src/harvester/types.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:54:15Z"
concept_id: crates/oxide-library/src/harvester/types/standard_chip
language: rust
---

# standard_chip

Creates standard 2-terminal SMD chip dimensions (e.g., 0402, 0603, 0805, 1206).

## Signature

```rust
impl PackageDimensions { pub fn standard_chip(length_mm: f64, width_mm: f64, height_mm: f64) -> Self }
```

## Visibility

- `pub`

## Docstring

Creates standard 2-terminal SMD chip dimensions (e.g., 0402, 0603, 0805, 1206).

## Source
Lines 222–234 in `crates/oxide-library/src/harvester/types.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [types](/crates/oxide-library/src/harvester/types.md) |
