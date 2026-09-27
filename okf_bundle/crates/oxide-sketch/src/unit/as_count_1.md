---
okf_version: "0.2"
type: Function
title: as_count
description: Return the raw scalar if this quantity is dimensionless.
resource: crates/oxide-sketch/src/unit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/unit/as_count_1
language: rust
---

# as_count

Return the raw scalar if this quantity is dimensionless.

## Signature

```rust
pub fn as_count(self) -> Result<f64, UnitError>
```

## Visibility

- `pub`

## Docstring

Return the raw scalar if this quantity is dimensionless.
Errors with [`UnitError::WrongFamily`] otherwise.

## Source
Lines 122–130 in `crates/oxide-sketch/src/unit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [unit](/crates/oxide-sketch/src/unit.md) |
