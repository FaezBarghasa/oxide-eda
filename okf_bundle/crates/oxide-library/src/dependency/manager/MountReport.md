---
okf_version: "0.2"
type: Class
title: MountReport
description: Mount summary for integrating resolved dependencies into an active EDA project.
resource: crates/oxide-library/src/dependency/manager.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:29:52Z"
concept_id: crates/oxide-library/src/dependency/manager/MountReport
language: rust
---

# MountReport

Mount summary for integrating resolved dependencies into an active EDA project.

## Signature

```rust
pub struct MountReport
```

## Decorators

- `derive(Debug, Clone, Default)`

## Visibility

- `pub`

## Docstring

Mount summary for integrating resolved dependencies into an active EDA project.
[derive(Debug, Clone, Default)]

## Methods

- `mounted_libraries`
- `footprint_search_paths`
- `component_search_paths`

## Source
Lines 32–39 in `crates/oxide-library/src/dependency/manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manager](/crates/oxide-library/src/dependency/manager.md) |
