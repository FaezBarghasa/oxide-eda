---
okf_version: "0.2"
type: Function
title: resolve_cascade_push
description: "Resolve a cascade push for a trace segment advancing through `spatial_index`."
resource: crates/oxide-router/src/interactive/conflict.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:06:43Z"
concept_id: crates/oxide-router/src/interactive/conflict/resolve_cascade_push_1
language: rust
---

# resolve_cascade_push

Resolve a cascade push for a trace segment advancing through `spatial_index`.

## Signature

```rust
pub fn resolve_cascade_push(
        &self,
        spatial_index: &SpatialIndex,
        trace_start: Point2D,
        trace_end: Point2D,
        net_id: u32,
        required_clearance: Microns,
    ) -> Vec<PushResult>
```

## Visibility

- `pub`

## Docstring

Resolve a cascade push for a trace segment advancing through `spatial_index`.

## Source
Lines 75–123 in `crates/oxide-router/src/interactive/conflict.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [conflict](/crates/oxide-router/src/interactive/conflict.md) |
| calls | [calculate_push](/crates/oxide-router/src/interactive/conflict/calculate_push.md) |
