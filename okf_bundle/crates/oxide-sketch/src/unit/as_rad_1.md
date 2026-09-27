---
okf_version: "0.2"
type: Function
title: as_rad
description: "Convert to radians. Errors with [`UnitError::WrongFamily`]"
resource: crates/oxide-sketch/src/unit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/unit/as_rad_1
language: rust
---

# as_rad

Convert to radians. Errors with [`UnitError::WrongFamily`]

## Signature

```rust
pub fn as_rad(self) -> Result<f64, UnitError>
```

## Visibility

- `pub`

## Docstring

Convert to radians. Errors with [`UnitError::WrongFamily`]
for non-angle units.

## Source
Lines 109–118 in `crates/oxide-sketch/src/unit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [unit](/crates/oxide-sketch/src/unit.md) |
