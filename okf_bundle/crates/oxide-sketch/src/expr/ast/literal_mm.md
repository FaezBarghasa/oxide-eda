---
okf_version: "0.2"
type: Function
title: literal_mm
description: Construct a length literal in millimetres.
resource: crates/oxide-sketch/src/expr/ast.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/ast/literal_mm
language: rust
---

# literal_mm

Construct a length literal in millimetres.

## Signature

```rust
impl ExprNode { pub fn literal_mm(value: f64) -> Self }
```

## Visibility

- `pub`

## Docstring

Construct a length literal in millimetres.

## Source
Lines 105–107 in `crates/oxide-sketch/src/expr/ast.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ast](/crates/oxide-sketch/src/expr/ast.md) |
