---
okf_version: "0.2"
type: Class
title: EditorContext
description: Hierarchical editor context for scoping commands and keybindings.
resource: crates/oxide-types/src/command.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:47:18Z"
concept_id: crates/oxide-types/src/command/EditorContext
language: rust
---

# EditorContext

Hierarchical editor context for scoping commands and keybindings.

## Signature

```rust
pub enum EditorContext
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)`
- `serde(rename_all = "snake_case")`

## Visibility

- `pub`

## Docstring

Hierarchical editor context for scoping commands and keybindings.
[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
[serde(rename_all = "snake_case")]

## Source
Lines 10–19 in `crates/oxide-types/src/command.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [command](/crates/oxide-types/src/command.md) |
