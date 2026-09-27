---
okf_version: "0.2"
type: Class
title: TransmissionLineCrossSection
description: Transmission Line Cross-Section Geometry (Microstrip / Stripline / CPW).
resource: crates/oxide-physics/src/bem_solver.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:35:08Z"
concept_id: crates/oxide-physics/src/bem_solver/TransmissionLineCrossSection
language: rust
---

# TransmissionLineCrossSection

Transmission Line Cross-Section Geometry (Microstrip / Stripline / CPW).

## Signature

```rust
pub struct TransmissionLineCrossSection
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Transmission Line Cross-Section Geometry (Microstrip / Stripline / CPW).
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `trace_width_m`
- `trace_thickness_m`
- `dielectric_height_m`
- `dielectric_er`
- `trace_spacing_m`

## Source
Lines 36–42 in `crates/oxide-physics/src/bem_solver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bem_solver](/crates/oxide-physics/src/bem_solver.md) |
