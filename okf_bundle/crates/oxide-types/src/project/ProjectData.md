---
okf_version: "0.2"
type: Class
title: ProjectData
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: crates/oxide-types/src/project.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:27:17Z"
concept_id: crates/oxide-types/src/project/ProjectData
language: rust
---

# ProjectData

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct ProjectData
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `name`
- `dir`
- `schematic_root`
- `pcb_file`
- `sheets`
- `variant_definitions`
- `active_variant`
- `libraries`
- `dependencies`
- `enable_git`

## Source
Lines 95–132 in `crates/oxide-types/src/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-types/src/project.md) |
