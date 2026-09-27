---
okf_version: "0.2"
type: Class
title: RuleViolationType
description: Categories of design rule violations.
resource: crates/oxide-rules/src/violation.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:27:50Z"
concept_id: crates/oxide-rules/src/violation/RuleViolationType
language: rust
---

# RuleViolationType

Categories of design rule violations.

## Signature

```rust
pub enum RuleViolationType
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)`
- `serde(rename_all = "snake_case")`

## Visibility

- `pub`

## Docstring

Categories of design rule violations.
[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
[serde(rename_all = "snake_case")]

## Source
Lines 10–26 in `crates/oxide-rules/src/violation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [violation](/crates/oxide-rules/src/violation.md) |
