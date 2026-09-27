---
okf_version: "0.2"
type: Function
title: family
description: Family this unit belongs to. Conversions between different
resource: crates/oxide-sketch/src/unit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/unit/family_1
language: rust
---

# family

Family this unit belongs to. Conversions between different

## Signature

```rust
pub fn family(self) -> UnitFamily
```

## Visibility

- `pub`

## Docstring

Family this unit belongs to. Conversions between different
families return [`UnitError::WrongFamily`].

## Source
Lines 40–46 in `crates/oxide-sketch/src/unit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [unit](/crates/oxide-sketch/src/unit.md) |
