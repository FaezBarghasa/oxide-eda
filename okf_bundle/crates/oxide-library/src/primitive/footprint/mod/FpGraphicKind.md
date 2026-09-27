---
okf_version: "0.2"
type: Class
title: FpGraphicKind
description: Footprint graphic kinds — silkscreen / fab outline primitives.
resource: crates/oxide-library/src/primitive/footprint/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/primitive/footprint/mod/FpGraphicKind
language: rust
---

# FpGraphicKind

Footprint graphic kinds — silkscreen / fab outline primitives.

## Signature

```rust
pub enum FpGraphicKind
```

## Decorators

- `derive(Clone, Debug, PartialEq, Serialize, Deserialize)`
- `serde(tag = "kind", rename_all = "snake_case")`

## Visibility

- `pub`

## Docstring

Footprint graphic kinds — silkscreen / fab outline primitives.
[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
[serde(tag = "kind", rename_all = "snake_case")]

## Methods

- `from`
- `to`
- `from`
- `to`
- `center`
- `radius`
- `center`
- `radius`
- `start_deg`
- `end_deg`
- `position`
- `content`
- `size`
- `frame`
- `vertices`

## Source
Lines 32–66 in `crates/oxide-library/src/primitive/footprint/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint](/crates/oxide-library/src/primitive/footprint/mod.md) |
