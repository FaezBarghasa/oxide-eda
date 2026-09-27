---
okf_version: "0.2"
type: Class
title: RuleDefinition
description: Metadata record that describes a rule — both built-in and DSL-compiled.
resource: crates/oxide-erc/src/rule.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-erc/src/rule/RuleDefinition
language: rust
---

# RuleDefinition

Metadata record that describes a rule — both built-in and DSL-compiled.

## Signature

```rust
pub struct RuleDefinition
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Metadata record that describes a rule — both built-in and DSL-compiled.
The rule engine uses this to filter applicability and scope before calling
the rule's evaluation function.
[derive(Debug, Clone)]

## Methods

- `id`
- `name`
- `description`
- `target`
- `scope`
- `applicability`
- `default_severity`

## Source
Lines 96–104 in `crates/oxide-erc/src/rule.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rule](/crates/oxide-erc/src/rule.md) |
