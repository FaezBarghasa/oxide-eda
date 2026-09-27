---
okf_version: "0.2"
type: Function
title: strip_leading_zeros
resource: crates/oxide-types/src/designator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/designator/strip_leading_zeros
language: rust
---

# strip_leading_zeros

## Signature

```rust
fn strip_leading_zeros(digits: &[u8]) -> &[u8]
```

## Source
Lines 37–41 in `crates/oxide-types/src/designator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [designator](/crates/oxide-types/src/designator.md) |
| called_by | [compare_digit_runs](/crates/oxide-types/src/designator/compare_digit_runs.md) |
