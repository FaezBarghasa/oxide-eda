---
okf_version: "0.2"
type: Function
title: anchor_obstacle_count
description: Count wire endpoints + label anchors within
resource: crates/oxide-engine/src/transform/autoplace.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-engine/src/transform/autoplace/anchor_obstacle_count
language: rust
---

# anchor_obstacle_count

Count wire endpoints + label anchors within

## Signature

```rust
fn anchor_obstacle_count(
    ax: f64,
    ay: f64,
    document: &oxide_types::schematic::SchematicSheet,
) -> u32
```

## Docstring

Count wire endpoints + label anchors within
`ANCHOR_AVOID_RADIUS_MM` of `(ax, ay)`. Used as a tie-break bias so
candidate sides crowded with wires / labels lose to cleaner sides.

## Source
Lines 286–305 in `crates/oxide-engine/src/transform/autoplace.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [autoplace](/crates/oxide-engine/src/transform/autoplace.md) |
| called_by | [autoplace_fields](/crates/oxide-engine/src/transform/autoplace/autoplace_fields.md) |
