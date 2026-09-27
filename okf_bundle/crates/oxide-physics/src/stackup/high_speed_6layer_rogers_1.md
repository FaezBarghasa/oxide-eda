---
okf_version: "0.2"
type: Function
title: high_speed_6layer_rogers
description: "High-speed 6-layer 1.6mm stackup with RO4350B high-frequency top dielectric:"
resource: crates/oxide-physics/src/stackup.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-physics/src/stackup/high_speed_6layer_rogers_1
language: rust
---

# high_speed_6layer_rogers

High-speed 6-layer 1.6mm stackup with RO4350B high-frequency top dielectric:

## Signature

```rust
pub fn high_speed_6layer_rogers() -> Self
```

## Visibility

- `pub`

## Docstring

High-speed 6-layer 1.6mm stackup with RO4350B high-frequency top dielectric:
L1 (High-Speed Signal) -> RO4350B (100µm) -> L2 (GND Plane) -> Core (400µm) ->
L3 (Signal) -> Prepreg (400µm) -> L4 (PWR Plane) -> Core (400µm) -> L5 (GND) -> Prepreg (100µm) -> L6 (Signal).

## Source
Lines 332–351 in `crates/oxide-physics/src/stackup.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [stackup](/crates/oxide-physics/src/stackup.md) |
