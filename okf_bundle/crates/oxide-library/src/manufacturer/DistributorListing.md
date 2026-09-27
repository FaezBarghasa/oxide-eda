---
okf_version: "0.2"
type: Class
title: DistributorListing
description: One distributor listing — where this MPN can be sourced.
resource: crates/oxide-library/src/manufacturer.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/manufacturer/DistributorListing
language: rust
---

# DistributorListing

One distributor listing — where this MPN can be sourced.

## Signature

```rust
pub struct DistributorListing
```

## Decorators

- `derive(Clone, Debug, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

One distributor listing — where this MPN can be sourced.

Lifted onto `Revision` directly per the refactor plan. Pricing snapshots
(the live cache that `DistributorAdapter` populates) stay in
`distributor.rs` and are referenced by mpn rather than embedded here.
[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]

## Methods

- `distributor`
- `sku`
- `url`
- `moq`

## Source
Lines 57–68 in `crates/oxide-library/src/manufacturer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manufacturer](/crates/oxide-library/src/manufacturer.md) |
