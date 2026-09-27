---
okf_version: "0.2"
type: Class
title: ProjectLockfile
description: "Deterministic lockfile format (`project.lock`) for reproducible EDA designs."
resource: crates/oxide-types/src/project.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:27:17Z"
concept_id: crates/oxide-types/src/project/ProjectLockfile
language: rust
---

# ProjectLockfile

Deterministic lockfile format (`project.lock`) for reproducible EDA designs.

## Signature

```rust
pub struct ProjectLockfile
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Deterministic lockfile format (`project.lock`) for reproducible EDA designs.
[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `version`
- `dependencies`

## Source
Lines 215–220 in `crates/oxide-types/src/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-types/src/project.md) |
