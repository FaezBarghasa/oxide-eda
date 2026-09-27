---
okf_version: "0.2"
type: Function
title: evaluate
description: Evaluate this predicate against a primitive context.
resource: crates/oxide-rules/src/query_dsl.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T20:11:46Z"
concept_id: crates/oxide-rules/src/query_dsl/evaluate
language: rust
---

# evaluate

Evaluate this predicate against a primitive context.

## Signature

```rust
impl QueryPredicate { pub fn evaluate(&self, ctx: &PrimitiveEvaluationContext) -> bool }
```

## Visibility

- `pub`

## Docstring

Evaluate this predicate against a primitive context.

## Source
Lines 68–120 in `crates/oxide-rules/src/query_dsl.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [query_dsl](/crates/oxide-rules/src/query_dsl.md) |
