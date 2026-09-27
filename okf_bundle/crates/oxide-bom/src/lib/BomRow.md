---
okf_version: "0.2"
type: Class
title: BomRow
description: A single row in the generated BOM table.
resource: crates/oxide-bom/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-bom"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-bom/src/lib/BomRow
language: rust
---

# BomRow

A single row in the generated BOM table.

## Signature

```rust
pub struct BomRow
```

## Decorators

- `derive(Debug, Clone, Default, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

A single row in the generated BOM table.
[derive(Debug, Clone, Default, Serialize, Deserialize)]

## Methods

- `references`
- `name`
- `qty`
- `fitted_qty`
- `not_fitted_qty`
- `value`
- `footprint`
- `lib_ref`
- `description`
- `custom`

## Source
Lines 163–174 in `crates/oxide-bom/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-bom/src/lib.md) |
