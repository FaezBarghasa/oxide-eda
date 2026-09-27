---
okf_version: "0.2"
type: Function
title: generate_meander
description: Generate accordion / trombone meander pattern with default params
resource: crates/oxide-router/src/interactive/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:13:12Z"
concept_id: crates/oxide-router/src/interactive/mod/generate_meander
language: rust
---

# generate_meander

Generate accordion / trombone meander pattern with default params

## Signature

```rust
impl InteractiveRouter { pub fn generate_meander(
        &self,
        start: Point2D,
        end: Point2D,
        extra_length: Microns,
        net_id: NetId,
        layer: LayerId,
        width: Microns,
    ) -> Vec<RouteSegment> }
```

## Visibility

- `pub`

## Docstring

Generate accordion / trombone meander pattern with default params

## Source
Lines 395–405 in `crates/oxide-router/src/interactive/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [interactive](/crates/oxide-router/src/interactive/mod.md) |
