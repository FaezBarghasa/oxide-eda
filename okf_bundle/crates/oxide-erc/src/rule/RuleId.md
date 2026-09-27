---
okf_version: "0.2"
type: Class
title: RuleId
description: "Stable, namespaced rule identifier."
resource: crates/oxide-erc/src/rule.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-erc/src/rule/RuleId
language: rust
---

# RuleId

Stable, namespaced rule identifier.

## Signature

```rust
pub struct RuleId
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Hash)`

## Visibility

- `pub`

## Docstring

Stable, namespaced rule identifier.
Built-in rules use the `"builtin::"` prefix; DSL rules use `"user::"`.
[derive(Debug, Clone, PartialEq, Eq, Hash)]

## Source
Lines 14–14 in `crates/oxide-erc/src/rule.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rule](/crates/oxide-erc/src/rule.md) |
| called_by | [builtin](/crates/oxide-erc/src/rule/builtin.md) |
| called_by | [user](/crates/oxide-erc/src/rule/user.md) |
