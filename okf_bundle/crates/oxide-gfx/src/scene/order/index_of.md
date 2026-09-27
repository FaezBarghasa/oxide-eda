---
okf_version: "0.2"
type: Function
title: index_of
resource: crates/oxide-gfx/src/scene/order.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-gfx/src/scene/order/index_of
language: rust
---

# index_of

## Signature

```rust
fn index_of(order: &[SceneBucket], bucket: SceneBucket) -> Option<usize>
```

## Source
Lines 113–115 in `crates/oxide-gfx/src/scene/order.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [order](/crates/oxide-gfx/src/scene/order.md) |
| called_by | [gpu_and_schematic_cpu_disagree_on_where_fills_go](/crates/oxide-gfx/src/scene/order/gpu_and_schematic_cpu_disagree_on_where_fills_go.md) |
| called_by | [overlays_composite_above_base_buckets](/crates/oxide-gfx/src/scene/order/overlays_composite_above_base_buckets.md) |
| called_by | [schematic_overlays_and_erc_markers_composite_above_base_buckets](/crates/oxide-gfx/src/scene/order/schematic_overlays_and_erc_markers_composite_above_base_buckets.md) |
