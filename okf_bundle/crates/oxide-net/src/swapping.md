---
okf_version: "0.2"
type: Module
title: swapping
description: "Topological Pin & Part Gate Swapping Engine."
resource: crates/oxide-net/src/swapping.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T20:13:08Z"
concept_id: crates/oxide-net/src/swapping
language: rust
---

# swapping

Topological Pin & Part Gate Swapping Engine.

## Docstring

Topological Pin & Part Gate Swapping Engine.

Conforms to Master Technical Directive §5.3:
- Net-Cross & Euclidean wirelength optimization for dense package breakouts (FPGAs, Quad Op-Amps).
- Logical equivalence groups for swappable pins and sub-part gates.
- Atomic Engineering Change Order (ECO) generation back-annotating changes to the schematic capture model.

## Relationships

| Type | Target |
|------|--------|
| related | [SwappablePin](/crates/oxide-net/src/swapping/SwappablePin.md) |
| related | [SwappableGate](/crates/oxide-net/src/swapping/SwappableGate.md) |
| related | [PinSwapAssignment](/crates/oxide-net/src/swapping/PinSwapAssignment.md) |
| related | [PinSwappingEngine](/crates/oxide-net/src/swapping/PinSwappingEngine.md) |
| related | [optimize_pin_swaps](/crates/oxide-net/src/swapping/optimize_pin_swaps.md) |
| related | [optimize_pin_swaps](/crates/oxide-net/src/swapping/optimize_pin_swaps.md) |
| related | [test_pin_swapping_optimization_and_eco_generation](/crates/oxide-net/src/swapping/test_pin_swapping_optimization_and_eco_generation.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
