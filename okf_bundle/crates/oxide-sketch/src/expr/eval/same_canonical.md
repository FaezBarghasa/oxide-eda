---
okf_version: "0.2"
type: Function
title: same_canonical
description: "Two quantities count as equal for `Lookup` matching iff they share"
resource: crates/oxide-sketch/src/expr/eval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/eval/same_canonical
language: rust
---

# same_canonical

Two quantities count as equal for `Lookup` matching iff they share

## Signature

```rust
fn same_canonical(a: Quantity, b: Quantity) -> Result<bool, ExprError>
```

## Docstring

Two quantities count as equal for `Lookup` matching iff they share
a family and their canonical values are within `EQ_TOL`.

## Source
Lines 396–402 in `crates/oxide-sketch/src/expr/eval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [eval](/crates/oxide-sketch/src/expr/eval.md) |
| calls | [canonical_pair](/crates/oxide-sketch/src/expr/eval/canonical_pair.md) |
| called_by | [eval_lookup](/crates/oxide-sketch/src/expr/eval/eval_lookup.md) |
