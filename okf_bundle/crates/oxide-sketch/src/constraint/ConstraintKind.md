---
okf_version: "0.2"
type: Class
title: ConstraintKind
description: "[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]"
resource: crates/oxide-sketch/src/constraint.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/constraint/ConstraintKind
language: rust
---

# ConstraintKind

[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]

## Signature

```rust
pub enum ConstraintKind
```

## Decorators

- `derive(Clone, Debug, PartialEq, Serialize, Deserialize)`
- `serde(tag = "kind", rename_all = "PascalCase")`

## Visibility

- `pub`

## Docstring

[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
[serde(tag = "kind", rename_all = "PascalCase")]

## Methods

- `p1`
- `p2`
- `point`
- `line`
- `point`
- `arc`
- `line`
- `line`
- `l1`
- `l2`
- `l1`
- `l2`
- `p1`
- `p2`
- `target`
- `point`
- `line`
- `target`
- `point`
- `circle`
- `target`
- `l1`
- `l2`
- `target`
- `l1`
- `l2`
- `e1`
- `e2`
- `line`
- `arc`
- `a1`
- `a2`
- `internal`
- `p1`
- `p2`
- `line`
- `p1`
- `p2`
- `center`
- `point`
- `line`
- `point`

## Source
Lines 27–119 in `crates/oxide-sketch/src/constraint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [constraint](/crates/oxide-sketch/src/constraint.md) |
