---
okf_version: "0.2"
type: Class
title: ChainError
description: "Why [`chain_into_closed_contour`] couldn't produce a single closed"
resource: crates/oxide-library/src/primitive/symbol/chain.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/chain/ChainError
language: rust
---

# ChainError

Why [`chain_into_closed_contour`] couldn't produce a single closed

## Signature

```rust
pub enum ChainError
```

## Decorators

- `derive(Clone, Copy, Debug, PartialEq, thiserror::Error)`

## Visibility

- `pub`

## Docstring

Why [`chain_into_closed_contour`] couldn't produce a single closed
ring from the given segments.
[derive(Clone, Copy, Debug, PartialEq, thiserror::Error)]

## Methods

- `segment_index`
- `segment_index`
- `gap_mm`
- `ends`
- `at`

## Source
Lines 99–139 in `crates/oxide-library/src/primitive/symbol/chain.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [chain](/crates/oxide-library/src/primitive/symbol/chain.md) |
