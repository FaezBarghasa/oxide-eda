---
okf_version: "0.2"
type: Function
title: rectangle_with_cutout
resource: crates/oxide-bake/src/cutout.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/cutout/rectangle_with_cutout
language: rust
---

# rectangle_with_cutout

## Signature

```rust
fn rectangle_with_cutout(attr: BoardCutoutAttr) -> SketchData
```

## Source
Lines 135–177 in `crates/oxide-bake/src/cutout.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cutout](/crates/oxide-bake/src/cutout.md) |
| called_by | [bake_cutout_edge_radius_evaluated](/crates/oxide-bake/src/cutout/bake_cutout_edge_radius_evaluated.md) |
| called_by | [bake_cutout_partial_depth_baked](/crates/oxide-bake/src/cutout/bake_cutout_partial_depth_baked.md) |
| called_by | [bake_cutout_simple](/crates/oxide-bake/src/cutout/bake_cutout_simple.md) |
