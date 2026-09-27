---
okf_version: "0.2"
type: Function
title: schematic_overlays_and_erc_markers_composite_above_base_buckets
description: "Overlays and ERC markers are presentation: they must land on top of"
resource: crates/oxide-gfx/src/scene/order.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-gfx/src/scene/order/schematic_overlays_and_erc_markers_composite_above_base_buckets
language: rust
---

# schematic_overlays_and_erc_markers_composite_above_base_buckets

Overlays and ERC markers are presentation: they must land on top of

## Signature

```rust
fn schematic_overlays_and_erc_markers_composite_above_base_buckets()
```

## Decorators

- `test`

## Docstring

Overlays and ERC markers are presentation: they must land on top of
every base bucket on the schematic replay, exactly as the PCB rule
above requires for its own overlays.
[test]

## Source
Lines 242–274 in `crates/oxide-gfx/src/scene/order.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [order](/crates/oxide-gfx/src/scene/order.md) |
| calls | [index_of](/crates/oxide-gfx/src/scene/order/index_of.md) |
