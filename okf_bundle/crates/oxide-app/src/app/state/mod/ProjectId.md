---
okf_version: "0.2"
type: Class
title: ProjectId
description: Opaque identifier for a loaded project in the workspace. Assigned by
resource: crates/oxide-app/src/app/state/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/state/mod/ProjectId
language: rust
---

# ProjectId

Opaque identifier for a loaded project in the workspace. Assigned by

## Signature

```rust
pub struct ProjectId
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)`

## Visibility

- `pub`

## Docstring

Opaque identifier for a loaded project in the workspace. Assigned by
`DocumentState::next_project_id` on load and never reused, so stale
references (e.g. a tab pointing at a closed project) resolve to `None`
via `DocumentState::project_by_id` instead of silently aliasing another
project.
[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]

## Source
Lines 240–240 in `crates/oxide-app/src/app/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/state/mod.md) |
| called_by | [mint_project_id](/crates/oxide-app/src/app/state/mod/mint_project_id.md) |
| called_by | [project](/crates/oxide-app/src/app/state/scope/project.md) |
