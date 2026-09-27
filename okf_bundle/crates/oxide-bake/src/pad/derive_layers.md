---
okf_version: "0.2"
type: Function
title: derive_layers
description: Layer set for a normal pad based on mounting style + copper side.
resource: crates/oxide-bake/src/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-bake/src/pad/derive_layers
language: rust
---

# derive_layers

Layer set for a normal pad based on mounting style + copper side.

## Signature

```rust
fn derive_layers(kind: PadKind, side: PadSide) -> Vec<LayerId>
```

## Docstring

Layer set for a normal pad based on mounting style + copper side.

Names produced by [`OxideLayer::altium_label`].

## Source
Lines 375–410 in `crates/oxide-bake/src/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-bake/src/pad.md) |
| calls | [fiducial_layers](/crates/oxide-bake/src/pad/fiducial_layers.md) |
| called_by | [bake_one_pad](/crates/oxide-bake/src/pad/bake_one_pad.md) |
