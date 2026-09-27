---
okf_version: "0.2"
type: Class
title: PrimitiveEvaluationContext
description: Query context describing an arbitrary PCB primitive being evaluated.
resource: crates/oxide-rules/src/query_dsl.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T20:11:46Z"
concept_id: crates/oxide-rules/src/query_dsl/PrimitiveEvaluationContext
language: rust
---

# PrimitiveEvaluationContext

Query context describing an arbitrary PCB primitive being evaluated.

## Signature

```rust
pub struct PrimitiveEvaluationContext
```

## Type Parameters

- `'a`

## Decorators

- `derive(Debug, Clone, Default)`

## Visibility

- `pub`

## Docstring

Query context describing an arbitrary PCB primitive being evaluated.
[derive(Debug, Clone, Default)]

## Methods

- `object_type`
- `net_name`
- `layer_name`
- `reference`
- `value`
- `footprint_id`
- `width_mm`
- `drill_mm`
- `diameter_mm`
- `is_locked`

## Source
Lines 53–64 in `crates/oxide-rules/src/query_dsl.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [query_dsl](/crates/oxide-rules/src/query_dsl.md) |
