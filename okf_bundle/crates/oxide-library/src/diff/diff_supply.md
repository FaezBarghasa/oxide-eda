---
okf_version: "0.2"
type: Function
title: diff_supply
resource: crates/oxide-library/src/diff.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/diff/diff_supply
language: rust
---

# diff_supply

## Signature

```rust
fn diff_supply(a: &[DistributorListing], b: &[DistributorListing]) -> ListDiff
```

## Source
Lines 219–227 in `crates/oxide-library/src/diff.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diff](/crates/oxide-library/src/diff.md) |
| called_by | [diff_rows](/crates/oxide-library/src/diff/diff_rows.md) |
