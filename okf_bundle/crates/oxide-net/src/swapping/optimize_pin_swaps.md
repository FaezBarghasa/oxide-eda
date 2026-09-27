---
okf_version: "0.2"
type: Function
title: optimize_pin_swaps
description: Computes optimal pin-to-net assignments to minimize wirelength and uncross routing paths.
resource: crates/oxide-net/src/swapping.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T20:13:08Z"
concept_id: crates/oxide-net/src/swapping/optimize_pin_swaps
language: rust
---

# optimize_pin_swaps

Computes optimal pin-to-net assignments to minimize wirelength and uncross routing paths.

## Signature

```rust
impl PinSwappingEngine { pub fn optimize_pin_swaps(
        component_ref: &str,
        pins: &[SwappablePin],
        target_destinations_mm: &[[f64; 2]],
    ) -> (Vec<PinSwapAssignment>, EcoReport) }
```

## Visibility

- `pub`

## Docstring

Computes optimal pin-to-net assignments to minimize wirelength and uncross routing paths.

## Source
Lines 43–97 in `crates/oxide-net/src/swapping.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [swapping](/crates/oxide-net/src/swapping.md) |
