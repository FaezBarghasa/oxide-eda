---
okf_version: "0.2"
type: Class
title: PinShapeStyle
description: Pin graphic decoration on the symbol pin tip.
resource: crates/oxide-types/src/schematic/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:15:42Z"
concept_id: crates/oxide-types/src/schematic/mod/PinShapeStyle
language: rust
---

# PinShapeStyle

Pin graphic decoration on the symbol pin tip.

## Signature

```rust
pub enum PinShapeStyle
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)`
- `serde(rename_all = "snake_case")`

## Visibility

- `pub`

## Docstring

Pin graphic decoration on the symbol pin tip.

7 variants — drops the per-direction "low" shape modifiers that
other EDA tools include (since `PinDirection`'s `OpenDrainLow` /
`OpenDrainHigh` carry that information already). Adds Schmitt /
Hysteresis as Oxide-original variants.
[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
[serde(rename_all = "snake_case")]

## Source
Lines 388–396 in `crates/oxide-types/src/schematic/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic](/crates/oxide-types/src/schematic/mod.md) |
