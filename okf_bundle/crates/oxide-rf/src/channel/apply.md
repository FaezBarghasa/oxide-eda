---
okf_version: "0.2"
type: Function
title: apply
description: Applies channel impairments and deterministic pseudo-random Gaussian noise.
resource: crates/oxide-rf/src/channel.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rf"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:27:05Z"
concept_id: crates/oxide-rf/src/channel/apply
language: rust
---

# apply

Applies channel impairments and deterministic pseudo-random Gaussian noise.

## Signature

```rust
impl ChannelModel { pub fn apply(&self, symbols: &[IqSymbol]) -> Vec<IqSymbol> }
```

## Visibility

- `pub`

## Docstring

Applies channel impairments and deterministic pseudo-random Gaussian noise.

## Source
Lines 29–58 in `crates/oxide-rf/src/channel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [channel](/crates/oxide-rf/src/channel.md) |
