---
okf_version: "0.2"
type: Class
title: DimTarget
description: Optional dimension target — either a literal length/angle in
resource: crates/oxide-sketch/src/constraint.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/constraint/DimTarget
language: rust
---

# DimTarget

Optional dimension target — either a literal length/angle in

## Signature

```rust
pub enum DimTarget
```

## Decorators

- `derive(Clone, Debug, PartialEq, Serialize, Deserialize)`
- `serde(untagged)`

## Visibility

- `pub`

## Docstring

Optional dimension target — either a literal length/angle in
canonical units (mm or rad) or an expression string evaluated at
solve time.

Phase 2 honours `Literal` only; `Expr` is preserved through the
schema and evaluated by the parser/evaluator added in Phase 4.
[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
[serde(untagged)]

## Source
Lines 13–16 in `crates/oxide-sketch/src/constraint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [constraint](/crates/oxide-sketch/src/constraint.md) |
