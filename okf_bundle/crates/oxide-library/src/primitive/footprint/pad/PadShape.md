---
okf_version: "0.2"
type: Class
title: PadShape
description: Pad geometry shape.
resource: crates/oxide-library/src/primitive/footprint/pad.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/footprint/pad/PadShape
language: rust
---

# PadShape

Pad geometry shape.

## Signature

```rust
pub enum PadShape
```

## Decorators

- `derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)`
- `serde(tag = "kind", rename_all = "snake_case")`

## Visibility

- `pub`

## Docstring

Pad geometry shape.
[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
[serde(tag = "kind", rename_all = "snake_case")]

## Methods

- `radius_ratio`
- `chamfer_ratio`
- `corners`

## Source
Lines 85–104 in `crates/oxide-library/src/primitive/footprint/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-library/src/primitive/footprint/pad.md) |
