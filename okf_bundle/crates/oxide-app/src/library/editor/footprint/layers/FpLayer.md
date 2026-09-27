---
okf_version: "0.2"
type: Class
title: FpLayer
description: One of the visible layers shown in the Footprint tab toolbar.
resource: crates/oxide-app/src/library/editor/footprint/layers.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/layers/FpLayer
language: rust
---

# FpLayer

One of the visible layers shown in the Footprint tab toolbar.

## Signature

```rust
pub enum FpLayer
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Hash)`

## Visibility

- `pub`

## Docstring

One of the visible layers shown in the Footprint tab toolbar.

The Standard/Altium layer model is much wider than this — F/B
paste, mask, adhesive, etc. all exist — but the MVP exposes only
the layers the user must see to draw a pad + courtyard. Extending
this enum is purely additive.
[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

## Source
Lines 16–27 in `crates/oxide-app/src/library/editor/footprint/layers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [layers](/crates/oxide-app/src/library/editor/footprint/layers.md) |
