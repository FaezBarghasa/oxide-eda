---
okf_version: "0.2"
type: Class
title: RuleScope
description: Hierarchical scope where a design constraint applies.
resource: crates/oxide-rules/src/scope.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:31:50Z"
concept_id: crates/oxide-rules/src/scope/RuleScope
language: rust
---

# RuleScope

Hierarchical scope where a design constraint applies.

## Signature

```rust
pub enum RuleScope
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Hash)`

## Visibility

- `pub`

## Docstring

Hierarchical scope where a design constraint applies.

Order of evaluation and specificity:
`Net` (highest, 4) > `NetClass` (3) > `Room` (2) > `Global` (lowest, 1).
[derive(Debug, Clone, PartialEq, Eq, Hash)]

## Source
Lines 10–19 in `crates/oxide-rules/src/scope.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scope](/crates/oxide-rules/src/scope.md) |
