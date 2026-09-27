---
okf_version: "0.2"
type: Class
title: WidthRule
description: "Copper trace routing width constraints (Min, Preferred, Max)."
resource: crates/oxide-rules/src/rules.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:26:29Z"
concept_id: crates/oxide-rules/src/rules/WidthRule
language: rust
---

# WidthRule

Copper trace routing width constraints (Min, Preferred, Max).

## Signature

```rust
pub struct WidthRule
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Copper trace routing width constraints (Min, Preferred, Max).
[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `scope`
- `min_width`
- `preferred_width`
- `max_width`

## Source
Lines 47–52 in `crates/oxide-rules/src/rules.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rules](/crates/oxide-rules/src/rules.md) |
