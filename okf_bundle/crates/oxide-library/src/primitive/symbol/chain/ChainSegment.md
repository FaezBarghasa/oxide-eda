---
okf_version: "0.2"
type: Class
title: ChainSegment
description: One input stroke to be chained. Mirrors the geometry-bearing fields
resource: crates/oxide-library/src/primitive/symbol/chain.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/chain/ChainSegment
language: rust
---

# ChainSegment

One input stroke to be chained. Mirrors the geometry-bearing fields

## Signature

```rust
pub enum ChainSegment
```

## Decorators

- `derive(Clone, Copy, Debug, PartialEq)`

## Visibility

- `pub`

## Docstring

One input stroke to be chained. Mirrors the geometry-bearing fields
of [`super::SymbolGraphicKind::Line`] / [`super::SymbolGraphicKind::Arc`]
so callers can build these directly from selected symbol graphics.
[derive(Clone, Copy, Debug, PartialEq)]

## Methods

- `from`
- `to`
- `center`
- `radius`
- `start_deg`
- `end_deg`

## Source
Lines 83–94 in `crates/oxide-library/src/primitive/symbol/chain.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [chain](/crates/oxide-library/src/primitive/symbol/chain.md) |
