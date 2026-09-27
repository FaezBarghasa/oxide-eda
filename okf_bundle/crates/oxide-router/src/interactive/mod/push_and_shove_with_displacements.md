---
okf_version: "0.2"
type: Function
title: push_and_shove_with_displacements
description: "Push-and-shove routing: generates direct / A* trace while computing obstacle displacement."
resource: crates/oxide-router/src/interactive/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:13:12Z"
concept_id: crates/oxide-router/src/interactive/mod/push_and_shove_with_displacements
language: rust
---

# push_and_shove_with_displacements

Push-and-shove routing: generates direct / A* trace while computing obstacle displacement.

## Signature

```rust
impl InteractiveRouter { pub fn push_and_shove_with_displacements(
        &self,
        start: Point2D,
        end: Point2D,
        net_id: NetId,
        layer: LayerId,
        width: Microns,
    ) -> (Vec<RouteSegment>, Vec<PushResult>) }
```

## Visibility

- `pub`

## Docstring

Push-and-shove routing: generates direct / A* trace while computing obstacle displacement.

## Source
Lines 277–296 in `crates/oxide-router/src/interactive/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [interactive](/crates/oxide-router/src/interactive/mod.md) |
