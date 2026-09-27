---
okf_version: "0.2"
type: Function
title: part_count_round_trips
description: "A declared multi-unit symbol with no pins keeps its `part_count`"
resource: crates/oxide-library/src/primitive/symbol/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/symbol/tests/part_count_round_trips
language: rust
---

# part_count_round_trips

A declared multi-unit symbol with no pins keeps its `part_count`

## Signature

```rust
fn part_count_round_trips()
```

## Decorators

- `test`

## Docstring

A declared multi-unit symbol with no pins keeps its `part_count`
across a TOML+TSV round-trip (the count is first-class, not derived
from pins alone).
[test]

## Source
Lines 413–420 in `crates/oxide-library/src/primitive/symbol/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-library/src/primitive/symbol/tests.md) |
