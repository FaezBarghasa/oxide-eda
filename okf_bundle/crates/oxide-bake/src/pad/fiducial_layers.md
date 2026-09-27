---
okf_version: "0.2"
type: Function
title: fiducial_layers
description: "Layer set for a Fiducial pad — copper + mask only, no paste."
resource: crates/oxide-bake/src/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-bake/src/pad/fiducial_layers
language: rust
---

# fiducial_layers

Layer set for a Fiducial pad — copper + mask only, no paste.

## Signature

```rust
fn fiducial_layers(side: PadSide) -> Vec<LayerId>
```

## Docstring

Layer set for a Fiducial pad — copper + mask only, no paste.

## Source
Lines 353–370 in `crates/oxide-bake/src/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-bake/src/pad.md) |
| called_by | [bake_one_pad](/crates/oxide-bake/src/pad/bake_one_pad.md) |
| called_by | [derive_layers](/crates/oxide-bake/src/pad/derive_layers.md) |
