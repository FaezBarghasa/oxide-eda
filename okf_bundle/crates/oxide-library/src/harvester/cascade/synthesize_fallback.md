---
okf_version: "0.2"
type: Function
title: synthesize_fallback
description: Synthesizes fallback component metadata if distributor APIs yield partial data.
resource: crates/oxide-library/src/harvester/cascade.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T13:01:27Z"
concept_id: crates/oxide-library/src/harvester/cascade/synthesize_fallback
language: rust
---

# synthesize_fallback

Synthesizes fallback component metadata if distributor APIs yield partial data.

## Signature

```rust
impl HarvesterCascade { pub fn synthesize_fallback(&self, query: &HarvestQuery) -> HarvestedRawData }
```

## Visibility

- `pub`

## Docstring

Synthesizes fallback component metadata if distributor APIs yield partial data.

## Source
Lines 42–101 in `crates/oxide-library/src/harvester/cascade.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cascade](/crates/oxide-library/src/harvester/cascade.md) |
