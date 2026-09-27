---
okf_version: "0.2"
type: Function
title: package_delay_to_length_microns
description: Convert internal package delay into equivalent copper track length in micrometers
resource: crates/oxide-types/src/net.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T04:59:29Z"
concept_id: crates/oxide-types/src/net/package_delay_to_length_microns_1
language: rust
---

# package_delay_to_length_microns

Convert internal package delay into equivalent copper track length in micrometers

## Signature

```rust
pub fn package_delay_to_length_microns(&self, ps_per_mm: f64) -> i64
```

## Visibility

- `pub`

## Docstring

Convert internal package delay into equivalent copper track length in micrometers
(assumes standard ~150 ps/inch = ~5.9 ps/mm => ~0.169 mm/ps = ~169.5 µm/ps propagation speed).

## Source
Lines 125–129 in `crates/oxide-types/src/net.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [net](/crates/oxide-types/src/net.md) |
