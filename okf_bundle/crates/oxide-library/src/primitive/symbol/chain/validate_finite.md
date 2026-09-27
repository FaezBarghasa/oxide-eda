---
okf_version: "0.2"
type: Function
title: validate_finite
description: "Reject any segment carrying a `NaN`/`inf` coordinate or radius up"
resource: crates/oxide-library/src/primitive/symbol/chain.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/chain/validate_finite
language: rust
---

# validate_finite

Reject any segment carrying a `NaN`/`inf` coordinate or radius up

## Signature

```rust
fn validate_finite(segments: &[ChainSegment]) -> Result<(), ChainError>
```

## Docstring

Reject any segment carrying a `NaN`/`inf` coordinate or radius up
front. Left unchecked, non-finite input propagates silently through
every downstream distance/angle computation and can surface as a
misleading [`ChainError::OpenChain`] (`gap_mm: NaN`) or
[`ChainError::Disjoint`] instead of the actual problem: bad input.

## Source
Lines 186–207 in `crates/oxide-library/src/primitive/symbol/chain.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [chain](/crates/oxide-library/src/primitive/symbol/chain.md) |
| called_by | [chain_into_closed_contour](/crates/oxide-library/src/primitive/symbol/chain/chain_into_closed_contour.md) |
