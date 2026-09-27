---
okf_version: "0.2"
type: Function
title: oxide_layer_id
description: "Build a `LayerId` from a `OxideLayer`, using its Altium-style"
resource: crates/oxide-bake/src/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-bake/src/pad/oxide_layer_id
language: rust
---

# oxide_layer_id

Build a `LayerId` from a `OxideLayer`, using its Altium-style

## Signature

```rust
fn oxide_layer_id(l: OxideLayer) -> LayerId
```

## Docstring

Build a `LayerId` from a `OxideLayer`, using its Altium-style
display label as the string-typed wrapper's content.

## Source
Lines 338–340 in `crates/oxide-bake/src/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-bake/src/pad.md) |
