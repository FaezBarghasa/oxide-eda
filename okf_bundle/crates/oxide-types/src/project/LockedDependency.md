---
okf_version: "0.2"
type: Class
title: LockedDependency
description: "A locked, exact resolution record stored in `project.lock`."
resource: crates/oxide-types/src/project.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:27:17Z"
concept_id: crates/oxide-types/src/project/LockedDependency
language: rust
---

# LockedDependency

A locked, exact resolution record stored in `project.lock`.

## Signature

```rust
pub struct LockedDependency
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

A locked, exact resolution record stored in `project.lock`.
[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `name`
- `kind`
- `url`
- `commit_oid`
- `tree_oid`
- `resolved_version`
- `install_path`
- `locked_at`

## Source
Lines 196–211 in `crates/oxide-types/src/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-types/src/project.md) |
