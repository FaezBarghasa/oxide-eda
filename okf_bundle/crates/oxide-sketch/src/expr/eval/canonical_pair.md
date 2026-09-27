---
okf_version: "0.2"
type: Function
title: canonical_pair
description: Convert two same-family quantities to canonical units (mm / rad /
resource: crates/oxide-sketch/src/expr/eval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/eval/canonical_pair
language: rust
---

# canonical_pair

Convert two same-family quantities to canonical units (mm / rad /

## Signature

```rust
fn canonical_pair(lhs: Quantity, rhs: Quantity) -> Result<(f64, f64), ExprError>
```

## Docstring

Convert two same-family quantities to canonical units (mm / rad /
raw) and return the pair.

## Source
Lines 385–392 in `crates/oxide-sketch/src/expr/eval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [eval](/crates/oxide-sketch/src/expr/eval.md) |
| called_by | [eval_compare](/crates/oxide-sketch/src/expr/eval/eval_compare.md) |
| called_by | [same_canonical](/crates/oxide-sketch/src/expr/eval/same_canonical.md) |
