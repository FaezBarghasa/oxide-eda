---
okf_version: "0.2"
type: Function
title: standard_4layer_1_6mm
description: "Industry-standard 4-layer 1.6mm (63 mil) JLC/PCBWay stackup:"
resource: crates/oxide-physics/src/stackup.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-physics/src/stackup/standard_4layer_1_6mm_1
language: rust
---

# standard_4layer_1_6mm

Industry-standard 4-layer 1.6mm (63 mil) JLC/PCBWay stackup:

## Signature

```rust
pub fn standard_4layer_1_6mm() -> Self
```

## Visibility

- `pub`

## Docstring

Industry-standard 4-layer 1.6mm (63 mil) JLC/PCBWay stackup:
L1 (Top Signal, 35µm) -> Prepreg 7628 (200µm) -> L2 (GND Plane, 35µm) ->
Core FR-4 (1065µm) -> L3 (PWR Plane, 35µm) -> Prepreg 7628 (200µm) -> L4 (Bottom Signal, 35µm).

## Source
Lines 313–327 in `crates/oxide-physics/src/stackup.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [stackup](/crates/oxide-physics/src/stackup.md) |
