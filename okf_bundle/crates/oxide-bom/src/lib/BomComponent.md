---
okf_version: "0.2"
type: Class
title: BomComponent
description: Normalized component candidate for BOM processing.
resource: crates/oxide-bom/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-bom"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-bom/src/lib/BomComponent
language: rust
---

# BomComponent

Normalized component candidate for BOM processing.

## Signature

```rust
pub struct BomComponent
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Normalized component candidate for BOM processing.
[derive(Debug, Clone)]

## Methods

- `reference`
- `name`
- `value`
- `footprint`
- `lib_ref`
- `description`
- `dnp`
- `in_bom`
- `on_board`
- `variant_fitted`
- `custom`

## Source
Lines 15–29 in `crates/oxide-bom/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-bom/src/lib.md) |
