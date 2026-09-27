---
okf_version: "0.2"
type: Function
title: calculate
description: Constructs dataset and calculates EVM metrics from received and ideal symbols.
resource: crates/oxide-rf/src/constellation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rf"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:26:57Z"
concept_id: crates/oxide-rf/src/constellation/calculate
language: rust
---

# calculate

Constructs dataset and calculates EVM metrics from received and ideal symbols.

## Signature

```rust
impl ConstellationDataset { pub fn calculate(received_symbols: &[IqSymbol], ideal_symbols: &[IqSymbol]) -> Self }
```

## Visibility

- `pub`

## Docstring

Constructs dataset and calculates EVM metrics from received and ideal symbols.

## Source
Lines 30–83 in `crates/oxide-rf/src/constellation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [constellation](/crates/oxide-rf/src/constellation.md) |
