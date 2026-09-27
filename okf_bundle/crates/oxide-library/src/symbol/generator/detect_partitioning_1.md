---
okf_version: "0.2"
type: Function
title: detect_partitioning
description: "Automatically detects partitioning strategy based on pin count, names, and description."
resource: crates/oxide-library/src/symbol/generator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T13:02:04Z"
concept_id: crates/oxide-library/src/symbol/generator/detect_partitioning_1
language: rust
---

# detect_partitioning

Automatically detects partitioning strategy based on pin count, names, and description.

## Signature

```rust
pub fn detect_partitioning(pins: &[DiscoveredPin], description: &str) -> PartitioningStrategy
```

## Visibility

- `pub`

## Docstring

Automatically detects partitioning strategy based on pin count, names, and description.

## Source
Lines 56–71 in `crates/oxide-library/src/symbol/generator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [generator](/crates/oxide-library/src/symbol/generator.md) |
