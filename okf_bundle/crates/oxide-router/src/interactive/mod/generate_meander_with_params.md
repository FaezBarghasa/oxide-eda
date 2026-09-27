---
okf_version: "0.2"
type: Function
title: generate_meander_with_params
description: Generate accordion / trombone meander pattern for length and phase-delay matching
resource: crates/oxide-router/src/interactive/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:13:12Z"
concept_id: crates/oxide-router/src/interactive/mod/generate_meander_with_params
language: rust
---

# generate_meander_with_params

Generate accordion / trombone meander pattern for length and phase-delay matching

## Signature

```rust
impl InteractiveRouter { pub fn generate_meander_with_params(
        &self,
        start: Point2D,
        end: Point2D,
        extra_length: Microns,
        net_id: NetId,
        layer: LayerId,
        width: Microns,
        amplitude_microns: Microns,
        pitch_microns: Microns,
    ) -> Vec<RouteSegment> }
```

## Visibility

- `pub`

## Docstring

Generate accordion / trombone meander pattern for length and phase-delay matching

## Source
Lines 408–493 in `crates/oxide-router/src/interactive/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [interactive](/crates/oxide-router/src/interactive/mod.md) |
