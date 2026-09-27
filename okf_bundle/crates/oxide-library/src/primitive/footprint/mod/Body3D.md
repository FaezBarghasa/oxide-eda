---
okf_version: "0.2"
type: Class
title: Body3D
description: "Embedded 3D body description. Lives on [`Footprint`] so two MPNs that share"
resource: crates/oxide-library/src/primitive/footprint/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/primitive/footprint/mod/Body3D
language: rust
---

# Body3D

Embedded 3D body description. Lives on [`Footprint`] so two MPNs that share

## Signature

```rust
pub struct Body3D
```

## Decorators

- `derive(Clone, Debug, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Embedded 3D body description. Lives on [`Footprint`] so two MPNs that share
a footprint also share the same procedural 3D render.
[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]

## Methods

- `shape`
- `height_mm`
- `offset_z_mm`
- `top_color`
- `side_color`
- `outline`

## Source
Lines 111–123 in `crates/oxide-library/src/primitive/footprint/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint](/crates/oxide-library/src/primitive/footprint/mod.md) |
