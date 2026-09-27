---
okf_version: "0.2"
type: Class
title: DesignRule
description: Top-level unified design rule variant.
resource: crates/oxide-rules/src/rules.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:26:29Z"
concept_id: crates/oxide-rules/src/rules/DesignRule
language: rust
---

# DesignRule

Top-level unified design rule variant.

## Signature

```rust
pub enum DesignRule
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`
- `serde(tag = "rule_type", rename_all = "snake_case")`

## Visibility

- `pub`

## Docstring

Top-level unified design rule variant.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
[serde(tag = "rule_type", rename_all = "snake_case")]

## Source
Lines 194–207 in `crates/oxide-rules/src/rules.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rules](/crates/oxide-rules/src/rules.md) |
