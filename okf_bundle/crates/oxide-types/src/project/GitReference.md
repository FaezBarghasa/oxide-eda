---
okf_version: "0.2"
type: Class
title: GitReference
description: Target Git reference or constraint for resolving a dependency.
resource: crates/oxide-types/src/project.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:27:17Z"
concept_id: crates/oxide-types/src/project/GitReference
language: rust
---

# GitReference

Target Git reference or constraint for resolving a dependency.

## Signature

```rust
pub enum GitReference
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)`
- `serde(rename_all = "snake_case")`

## Visibility

- `pub`

## Docstring

Target Git reference or constraint for resolving a dependency.
[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
[serde(rename_all = "snake_case")]

## Source
Lines 153–162 in `crates/oxide-types/src/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-types/src/project.md) |
