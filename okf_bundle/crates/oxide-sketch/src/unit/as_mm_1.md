---
okf_version: "0.2"
type: Function
title: as_mm
description: "Convert to millimetres. Errors with [`UnitError::WrongFamily`]"
resource: crates/oxide-sketch/src/unit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/unit/as_mm_1
language: rust
---

# as_mm

Convert to millimetres. Errors with [`UnitError::WrongFamily`]

## Signature

```rust
pub fn as_mm(self) -> Result<f64, UnitError>
```

## Visibility

- `pub`

## Docstring

Convert to millimetres. Errors with [`UnitError::WrongFamily`]
for non-length units.

## Source
Lines 94–105 in `crates/oxide-sketch/src/unit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [unit](/crates/oxide-sketch/src/unit.md) |
