---
okf_version: "0.2"
type: Class
title: PriceBreak
description: "Price-break tier for one distributor — `qty @ unit_price_usd`."
resource: crates/oxide-library/src/distributor.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/distributor/PriceBreak
language: rust
---

# PriceBreak

Price-break tier for one distributor — `qty @ unit_price_usd`.

## Signature

```rust
pub struct PriceBreak
```

## Decorators

- `derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)`

## Visibility

- `pub`

## Docstring

Price-break tier for one distributor — `qty @ unit_price_usd`.

Lifted out of the legacy `embed.rs` module by the v0.9 library refactor;
pricing is a runtime/cache concept, not a static `Component` field, so it
belongs alongside the distributor adapters.
[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]

## Methods

- `qty`
- `unit_price_usd`

## Source
Lines 13–16 in `crates/oxide-library/src/distributor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [distributor](/crates/oxide-library/src/distributor.md) |
