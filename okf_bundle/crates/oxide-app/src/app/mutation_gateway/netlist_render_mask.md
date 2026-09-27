---
okf_version: "0.2"
type: Function
title: netlist_render_mask
description: "The `RenderInvalidation` bits that mean project connectivity changed, so"
resource: crates/oxide-app/src/app/mutation_gateway.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/app/mutation_gateway/netlist_render_mask
language: rust
---

# netlist_render_mask

The `RenderInvalidation` bits that mean project connectivity changed, so

## Signature

```rust
impl Oxide { fn netlist_render_mask() -> crate::schematic_runtime::RenderInvalidation }
```

## Docstring

The `RenderInvalidation` bits that mean project connectivity changed, so
the cached netlist must be re-derived. Mirrors the electrical
`DocumentPatch` bits: symbols, wires, labels, junctions, child sheets,
and — easy to miss — no-connects and buses, which `point_is_connected`
reads when deciding whether a pin lands on a net.

## Source
Lines 232–242 in `crates/oxide-app/src/app/mutation_gateway.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mutation_gateway](/crates/oxide-app/src/app/mutation_gateway.md) |
