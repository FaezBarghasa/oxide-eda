---
okf_version: "0.2"
type: Function
title: calculate_push
description: Calculate push displacement vector to move obstacle out of clearance zone.
resource: crates/oxide-router/src/interactive/conflict.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:06:43Z"
concept_id: crates/oxide-router/src/interactive/conflict/calculate_push
language: rust
---

# calculate_push

Calculate push displacement vector to move obstacle out of clearance zone.

## Signature

```rust
pub fn calculate_push(
    obstacle: &SpatialObject,
    trace_start: Point2D,
    trace_end: Point2D,
    required_clearance: Microns,
) -> Option<PushResult>
```

## Visibility

- `pub`

## Docstring

Calculate push displacement vector to move obstacle out of clearance zone.

## Source
Lines 19–47 in `crates/oxide-router/src/interactive/conflict.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [conflict](/crates/oxide-router/src/interactive/conflict.md) |
| called_by | [resolve_cascade_push](/crates/oxide-router/src/interactive/conflict/resolve_cascade_push.md) |
