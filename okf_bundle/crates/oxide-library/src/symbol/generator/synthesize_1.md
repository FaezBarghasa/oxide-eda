---
okf_version: "0.2"
type: Function
title: synthesize
description: "Synthesizes a standardized, electrically partitioned multi-gate symbol."
resource: crates/oxide-library/src/symbol/generator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T13:02:04Z"
concept_id: crates/oxide-library/src/symbol/generator/synthesize_1
language: rust
---

# synthesize

Synthesizes a standardized, electrically partitioned multi-gate symbol.

## Signature

```rust
pub fn synthesize(
        name: &str,
        designator_prefix: &str,
        description: &str,
        pins: &[DiscoveredPin],
    ) -> Symbol
```

## Visibility

- `pub`

## Docstring

Synthesizes a standardized, electrically partitioned multi-gate symbol.

## Source
Lines 74–98 in `crates/oxide-library/src/symbol/generator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [generator](/crates/oxide-library/src/symbol/generator.md) |
