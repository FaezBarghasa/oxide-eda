---
okf_version: "0.2"
type: Class
title: PricingSnapshot
description: "Captured pricing snapshot for a single MPN, partitioned by distributor."
resource: crates/oxide-library/src/distributor.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/distributor/PricingSnapshot
language: rust
---

# PricingSnapshot

Captured pricing snapshot for a single MPN, partitioned by distributor.

## Signature

```rust
pub struct PricingSnapshot
```

## Decorators

- `derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)`

## Visibility

- `pub`

## Docstring

Captured pricing snapshot for a single MPN, partitioned by distributor.
[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]

## Methods

- `captured_at`
- `by_distributor`

## Source
Lines 20–23 in `crates/oxide-library/src/distributor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [distributor](/crates/oxide-library/src/distributor.md) |
