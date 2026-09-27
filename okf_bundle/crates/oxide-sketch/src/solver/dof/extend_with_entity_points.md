---
okf_version: "0.2"
type: Function
title: extend_with_entity_points
description: "Append every Point reachable from `entity_id` (through Line/Arc/"
resource: crates/oxide-sketch/src/solver/dof.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-sketch/src/solver/dof/extend_with_entity_points
language: rust
---

# extend_with_entity_points

Append every Point reachable from `entity_id` (through Line/Arc/

## Signature

```rust
fn extend_with_entity_points(
    entity_id: SketchEntityId,
    sketch: &SketchData,
    out: &mut Vec<SketchEntityId>,
)
```

## Docstring

Append every Point reachable from `entity_id` (through Line/Arc/
Circle endpoints) to `out`.

## Source
Lines 297–308 in `crates/oxide-sketch/src/solver/dof.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dof](/crates/oxide-sketch/src/solver/dof.md) |
| called_by | [points_touched](/crates/oxide-sketch/src/solver/dof/points_touched.md) |
