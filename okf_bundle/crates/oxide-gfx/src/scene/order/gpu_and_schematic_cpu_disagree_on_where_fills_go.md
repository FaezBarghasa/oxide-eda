---
okf_version: "0.2"
type: Function
title: gpu_and_schematic_cpu_disagree_on_where_fills_go
description: "The divergence this whole module exists to make visible, now stated for"
resource: crates/oxide-gfx/src/scene/order.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-gfx/src/scene/order/gpu_and_schematic_cpu_disagree_on_where_fills_go
language: rust
---

# gpu_and_schematic_cpu_disagree_on_where_fills_go

The divergence this whole module exists to make visible, now stated for

## Signature

```rust
fn gpu_and_schematic_cpu_disagree_on_where_fills_go()
```

## Decorators

- `test`

## Docstring

The divergence this whole module exists to make visible, now stated for
the schematic too: the GPU composites fills first, the schematic CPU
replay composites them fourth. Symbol body fills are polygons and wires
are lines, so the two paths stack them opposite ways.

Deliberately asserts the divergence rather than parity — reconciling it
is a visual-authority call (see the module docs). When that call lands,
this test is the one that must be rewritten, which is the point: the
difference cannot be closed on one side only.
[test]

## Source
Lines 286–297 in `crates/oxide-gfx/src/scene/order.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [order](/crates/oxide-gfx/src/scene/order.md) |
| calls | [index_of](/crates/oxide-gfx/src/scene/order/index_of.md) |
