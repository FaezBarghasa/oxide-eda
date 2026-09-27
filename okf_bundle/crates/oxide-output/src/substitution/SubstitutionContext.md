---
okf_version: "0.2"
type: Class
title: SubstitutionContext
description: "Binds tokens to values for a single render pass — one sheet, one project"
resource: crates/oxide-output/src/substitution.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/src/substitution/SubstitutionContext
language: rust
---

# SubstitutionContext

Binds tokens to values for a single render pass — one sheet, one project

## Signature

```rust
pub struct SubstitutionContext
```

## Type Parameters

- `'a`

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Binds tokens to values for a single render pass — one sheet, one project
snapshot. Constructed per-export in the app layer.
[derive(Debug, Clone)]

## Methods

- `metadata`
- `filename`
- `sheet_name`
- `sheet_number`
- `sheet_count`
- `oxide_version`
- `variant`
- `physical_structure`
- `physical_sheet_number`
- `physical_document_number`

## Source
Lines 19–38 in `crates/oxide-output/src/substitution.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [substitution](/crates/oxide-output/src/substitution.md) |
