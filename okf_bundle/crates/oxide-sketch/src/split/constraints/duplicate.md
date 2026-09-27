---
okf_version: "0.2"
type: Function
title: duplicate
resource: crates/oxide-sketch/src/split/constraints.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/split/constraints/duplicate
language: rust
---

# duplicate

## Signature

```rust
fn duplicate(id: ConstraintId, a: ConstraintKind, b: ConstraintKind) -> Vec<Constraint>
```

## Source
Lines 95–103 in `crates/oxide-sketch/src/split/constraints.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [constraints](/crates/oxide-sketch/src/split/constraints.md) |
| called_by | [split_duplicated](/crates/oxide-sketch/src/split/constraints/split_duplicated.md) |
