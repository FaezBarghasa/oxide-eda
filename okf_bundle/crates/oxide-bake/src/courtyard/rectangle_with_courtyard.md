---
okf_version: "0.2"
type: Function
title: rectangle_with_courtyard
description: Build a 1×1 mm rectangle of 4 Lines and tag the first Line with
resource: crates/oxide-bake/src/courtyard.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/courtyard/rectangle_with_courtyard
language: rust
---

# rectangle_with_courtyard

Build a 1×1 mm rectangle of 4 Lines and tag the first Line with

## Signature

```rust
fn rectangle_with_courtyard() -> (SketchData, SketchEntityId)
```

## Docstring

Build a 1×1 mm rectangle of 4 Lines and tag the first Line with
CourtyardAttr. Returns the sketch + the seed Line ID.

## Source
Lines 110–152 in `crates/oxide-bake/src/courtyard.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [courtyard](/crates/oxide-bake/src/courtyard.md) |
| called_by | [bake_courtyard_construction_seed_skipped](/crates/oxide-bake/src/courtyard/bake_courtyard_construction_seed_skipped.md) |
| called_by | [bake_courtyard_rectangle](/crates/oxide-bake/src/courtyard/bake_courtyard_rectangle.md) |
| called_by | [bake_courtyard_second_attr_warns](/crates/oxide-bake/src/courtyard/bake_courtyard_second_attr_warns.md) |
