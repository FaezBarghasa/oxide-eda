---
okf_version: "0.2"
type: Function
title: compare_digit_runs
description: Compare two digit runs by numeric value.
resource: crates/oxide-types/src/designator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/designator/compare_digit_runs
language: rust
---

# compare_digit_runs

Compare two digit runs by numeric value.

## Signature

```rust
fn compare_digit_runs(a: &[u8], b: &[u8]) -> Ordering
```

## Docstring

Compare two digit runs by numeric value.

Leading zeros are stripped and the significant lengths compared first, so
arbitrarily long numeric tails stay correctly ordered where `parse::<u64>()`
would overflow and mis-order.

## Source
Lines 28–35 in `crates/oxide-types/src/designator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [designator](/crates/oxide-types/src/designator.md) |
| calls | [strip_leading_zeros](/crates/oxide-types/src/designator/strip_leading_zeros.md) |
| called_by | [compare_references](/crates/oxide-types/src/designator/compare_references.md) |
