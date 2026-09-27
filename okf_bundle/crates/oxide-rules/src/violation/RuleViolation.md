---
okf_version: "0.2"
type: Class
title: RuleViolation
description: Detailed design rule violation report with actionable delta metrics.
resource: crates/oxide-rules/src/violation.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:27:50Z"
concept_id: crates/oxide-rules/src/violation/RuleViolation
language: rust
---

# RuleViolation

Detailed design rule violation report with actionable delta metrics.

## Signature

```rust
pub struct RuleViolation
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Detailed design rule violation report with actionable delta metrics.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `violation_type`
- `message`
- `scope`
- `required_value`
- `actual_value`
- `location`
- `object_ids`

## Source
Lines 30–38 in `crates/oxide-rules/src/violation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [violation](/crates/oxide-rules/src/violation.md) |
