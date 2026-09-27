---
okf_version: "0.2"
type: Class
title: RuleConfigFile
description: "Top-level schema for `rules.toml` configuration files."
resource: crates/oxide-rules/src/manager.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:29:13Z"
concept_id: crates/oxide-rules/src/manager/RuleConfigFile
language: rust
---

# RuleConfigFile

Top-level schema for `rules.toml` configuration files.

## Signature

```rust
pub struct RuleConfigFile
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Top-level schema for `rules.toml` configuration files.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `version`
- `profile_name`
- `rules`

## Source
Lines 576–582 in `crates/oxide-rules/src/manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manager](/crates/oxide-rules/src/manager.md) |
