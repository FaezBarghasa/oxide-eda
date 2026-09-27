---
okf_version: "0.2"
type: Class
title: Sign
description: "Discrete sign of a predicate. `Zero` means \"within tolerance of"
resource: crates/oxide-sketch/src/geom/predicates.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/predicates/Sign
language: rust
---

# Sign

Discrete sign of a predicate. `Zero` means "within tolerance of

## Signature

```rust
pub enum Sign
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Discrete sign of a predicate. `Zero` means "within tolerance of
the boundary" — callers branch on the three cases the same way
they would on the `Ordering` of a comparison.
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Source
Lines 16–25 in `crates/oxide-sketch/src/geom/predicates.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [predicates](/crates/oxide-sketch/src/geom/predicates.md) |
