---
okf_version: "0.2"
type: Function
title: new
description: Convenience constructor — distributor + SKU only.
resource: crates/oxide-library/src/manufacturer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/manufacturer/new
language: rust
---

# new

Convenience constructor — distributor + SKU only.

## Signature

```rust
impl DistributorListing { pub fn new(distributor: impl Into<String>, sku: impl Into<String>) -> Self }
```

## Visibility

- `pub`

## Docstring

Convenience constructor — distributor + SKU only.

## Source
Lines 72–79 in `crates/oxide-library/src/manufacturer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manufacturer](/crates/oxide-library/src/manufacturer.md) |
