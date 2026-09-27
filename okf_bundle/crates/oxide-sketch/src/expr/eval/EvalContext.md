---
okf_version: "0.2"
type: Class
title: EvalContext
description: "Per-evaluation context — the parameter table (raw AST values, so"
resource: crates/oxide-sketch/src/expr/eval.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/eval/EvalContext
language: rust
---

# EvalContext

Per-evaluation context — the parameter table (raw AST values, so

## Signature

```rust
pub struct EvalContext
```

## Decorators

- `derive(Clone, Debug, Default)`

## Visibility

- `pub`

## Docstring

Per-evaluation context — the parameter table (raw AST values, so
references resolve recursively) plus the optional `(i, j)` array
index for inside an array expansion.
[derive(Clone, Debug, Default)]

## Methods

- `params`
- `array_index`

## Source
Lines 51–59 in `crates/oxide-sketch/src/expr/eval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [eval](/crates/oxide-sketch/src/expr/eval.md) |
