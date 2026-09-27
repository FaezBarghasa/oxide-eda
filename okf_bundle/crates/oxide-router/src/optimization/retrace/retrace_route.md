---
okf_version: "0.2"
type: Function
title: retrace_route
description: Retrace an existing route with current spatial index and clearance rules.
resource: crates/oxide-router/src/optimization/retrace.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:58:57Z"
concept_id: crates/oxide-router/src/optimization/retrace/retrace_route
language: rust
---

# retrace_route

Retrace an existing route with current spatial index and clearance rules.

## Signature

```rust
impl RetraceOptimizer { pub fn retrace_route(
        &self,
        route: &RoutingPath,
        spatial_index: &SpatialIndex,
    ) -> RoutingResult }
```

## Visibility

- `pub`

## Docstring

Retrace an existing route with current spatial index and clearance rules.

## Source
Lines 22–60 in `crates/oxide-router/src/optimization/retrace.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [retrace](/crates/oxide-router/src/optimization/retrace.md) |
| calls | [find_astar_path](/crates/oxide-router/src/interactive/astar/find_astar_path.md) |
