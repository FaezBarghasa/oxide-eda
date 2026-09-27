---
okf_version: "0.2"
type: Function
title: direct_route
description: "--- Specific Mode Implementations ---"
resource: crates/oxide-router/src/interactive/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:13:12Z"
concept_id: crates/oxide-router/src/interactive/mod/direct_route
language: rust
---

# direct_route

--- Specific Mode Implementations ---

## Signature

```rust
impl InteractiveRouter { fn direct_route(
        &self,
        start: Point2D,
        end: Point2D,
        net_id: NetId,
        layer: LayerId,
        width: Microns,
    ) -> Vec<RouteSegment> }
```

## Docstring

--- Specific Mode Implementations ---

## Source
Lines 210–226 in `crates/oxide-router/src/interactive/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [interactive](/crates/oxide-router/src/interactive/mod.md) |
