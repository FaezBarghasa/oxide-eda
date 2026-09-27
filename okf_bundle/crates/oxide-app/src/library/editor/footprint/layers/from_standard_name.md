---
okf_version: "0.2"
type: Function
title: from_standard_name
description: "Convert a Standard layer-name string into our enum, returning"
resource: crates/oxide-app/src/library/editor/footprint/layers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/layers/from_standard_name
language: rust
---

# from_standard_name

Convert a Standard layer-name string into our enum, returning

## Signature

```rust
impl FpLayer { pub fn from_standard_name(name: &str) -> Option<FpLayer> }
```

## Visibility

- `pub`

## Docstring

Convert a Standard layer-name string into our enum, returning
`None` for any layer the MVP doesn't expose. Used when parsing
a footprint sexpr — graphics on unknown layers are still drawn
(they end up routed to a sensible default colour) but the
toolbar can't toggle them.

## Source
Lines 74–85 in `crates/oxide-app/src/library/editor/footprint/layers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [layers](/crates/oxide-app/src/library/editor/footprint/layers.md) |
