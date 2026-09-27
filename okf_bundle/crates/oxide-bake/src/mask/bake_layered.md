---
okf_version: "0.2"
type: Function
title: bake_layered
description: "Generic helper: for every non-construction Line entity where"
resource: crates/oxide-bake/src/mask.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/mask/bake_layered
language: rust
---

# bake_layered

Generic helper: for every non-construction Line entity where

## Signature

```rust
fn bake_layered(
    sketch: &SketchData,
    solve: &FullSolveOutput,
    out: &mut Vec<O>,
    warnings: &mut Vec<String>,
    attr_extract: impl Fn(&oxide_sketch::entity::Entity) -> Option<OxideLayer>,
    make: F,
    attr_name: &'static str,
) -> Result<(), SketchError>
```

## Type Parameters

- `O`
- `F`

## Docstring

Generic helper: for every non-construction Line entity where
`attr_extract` returns Some(layer), trace the closed profile and
push `make(boundary, layer_id)` onto `out`. Skipped entities
produce a warning that names the attr (`attr_name`).

## Source
Lines 81–138 in `crates/oxide-bake/src/mask.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mask](/crates/oxide-bake/src/mask.md) |
| calls | [trace_closed_profile](/crates/oxide-bake/src/profile/trace_closed_profile.md) |
