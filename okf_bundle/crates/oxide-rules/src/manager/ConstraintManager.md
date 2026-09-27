---
okf_version: "0.2"
type: Class
title: ConstraintManager
description: Central constraint repository that resolves design rules hierarchically.
resource: crates/oxide-rules/src/manager.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:29:13Z"
concept_id: crates/oxide-rules/src/manager/ConstraintManager
language: rust
---

# ConstraintManager

Central constraint repository that resolves design rules hierarchically.

## Signature

```rust
pub struct ConstraintManager
```

## Decorators

- `derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Central constraint repository that resolves design rules hierarchically.

Rules are evaluated in strict order of specificity:
`Net` (4) > `NetClass` (3) > `Room` (2) > `Global` (1).
[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]

## Methods

- `rules`

## Source
Lines 20–22 in `crates/oxide-rules/src/manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manager](/crates/oxide-rules/src/manager.md) |
