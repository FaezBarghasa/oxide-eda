---
okf_version: "0.2"
type: Class
title: DesignCommand
description: "[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]"
resource: crates/oxide-types/src/command.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:47:18Z"
concept_id: crates/oxide-types/src/command/DesignCommand
language: rust
---

# DesignCommand

[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Signature

```rust
pub enum DesignCommand
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`
- `serde(tag = "target", content = "action", rename_all = "snake_case")`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
[serde(tag = "target", content = "action", rename_all = "snake_case")]

## Source
Lines 145–148 in `crates/oxide-types/src/command.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [command](/crates/oxide-types/src/command.md) |
