---
okf_version: "0.2"
type: Function
title: bake_courtyard
description: Bake the first CourtyardAttr-tagged closed profile into
resource: crates/oxide-bake/src/courtyard.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/courtyard/bake_courtyard
language: rust
---

# bake_courtyard

Bake the first CourtyardAttr-tagged closed profile into

## Signature

```rust
pub fn bake_courtyard(
    sketch: &SketchData,
    solve: &FullSolveOutput,
    courtyard_out: &mut Polygon,
    warnings: &mut Vec<String>,
) -> Result<(), SketchError>
```

## Visibility

- `pub`

## Docstring

Bake the first CourtyardAttr-tagged closed profile into
`courtyard_out`. The first non-construction Line entity carrying
CourtyardAttr is used as the trace seed.

Returns Ok even when the trace fails — failures are reported via
`warnings` so the bake pipeline can continue.

## Source
Lines 32–90 in `crates/oxide-bake/src/courtyard.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [courtyard](/crates/oxide-bake/src/courtyard.md) |
| calls | [trace_closed_profile](/crates/oxide-bake/src/profile/trace_closed_profile.md) |
| called_by | [solve_and_bake](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/solve_and_bake.md) |
| called_by | [bake_courtyard_construction_seed_skipped](/crates/oxide-bake/src/courtyard/bake_courtyard_construction_seed_skipped.md) |
| called_by | [bake_courtyard_open_chain_warns](/crates/oxide-bake/src/courtyard/bake_courtyard_open_chain_warns.md) |
| called_by | [bake_courtyard_rectangle](/crates/oxide-bake/src/courtyard/bake_courtyard_rectangle.md) |
| called_by | [bake_courtyard_second_attr_warns](/crates/oxide-bake/src/courtyard/bake_courtyard_second_attr_warns.md) |
