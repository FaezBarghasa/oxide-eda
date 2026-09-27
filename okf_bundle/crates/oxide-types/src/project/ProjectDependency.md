---
okf_version: "0.2"
type: Class
title: ProjectDependency
description: "A versioned dependency declared in `.snxprj`."
resource: crates/oxide-types/src/project.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:27:17Z"
concept_id: crates/oxide-types/src/project/ProjectDependency
language: rust
---

# ProjectDependency

A versioned dependency declared in `.snxprj`.

## Signature

```rust
pub struct ProjectDependency
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

A versioned dependency declared in `.snxprj`.
[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `name`
- `kind`
- `source`
- `subpath`
- `enabled`

## Source
Lines 175–188 in `crates/oxide-types/src/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-types/src/project.md) |
