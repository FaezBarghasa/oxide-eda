---
okf_version: "0.2"
type: Class
title: BomRuleOptions
description: Rule-level on/off switches for BOM validation.
resource: crates/oxide-bom/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-bom"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-bom/src/lib/BomRuleOptions
language: rust
---

# BomRuleOptions

Rule-level on/off switches for BOM validation.

## Signature

```rust
pub struct BomRuleOptions
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Rule-level on/off switches for BOM validation.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `missing_footprint`
- `missing_mpn`
- `duplicate_designator`
- `empty_or_zero_qty`

## Source
Lines 90–95 in `crates/oxide-bom/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-bom/src/lib.md) |
