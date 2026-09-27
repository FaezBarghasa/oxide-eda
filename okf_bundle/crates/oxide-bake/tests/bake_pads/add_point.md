---
okf_version: "0.2"
type: Function
title: add_point
description: "Add a Point and return its ID. The `pad` attr can be attached"
resource: crates/oxide-bake/tests/bake_pads.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/tests/bake_pads/add_point
language: rust
---

# add_point

Add a Point and return its ID. The `pad` attr can be attached

## Signature

```rust
impl Sketch { fn add_point(&mut self, x: f64, y: f64) -> SketchEntityId }
```

## Docstring

Add a Point and return its ID. The `pad` attr can be attached
afterwards by writing into `data.entities[idx].pad`.

## Source
Lines 55–61 in `crates/oxide-bake/tests/bake_pads.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bake_pads](/crates/oxide-bake/tests/bake_pads.md) |
