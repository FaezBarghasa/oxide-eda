---
okf_version: "0.2"
type: Class
title: MenuCommand
description: Unified menu command enum covering all document and system actions.
resource: crates/oxide-types/src/command.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:47:18Z"
concept_id: crates/oxide-types/src/command/MenuCommand
language: rust
---

# MenuCommand

Unified menu command enum covering all document and system actions.

## Signature

```rust
pub enum MenuCommand
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`
- `serde(tag = "category", content = "command", rename_all = "snake_case")`

## Visibility

- `pub`

## Docstring

Unified menu command enum covering all document and system actions.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
[serde(tag = "category", content = "command", rename_all = "snake_case")]

## Source
Lines 24–35 in `crates/oxide-types/src/command.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [command](/crates/oxide-types/src/command.md) |
