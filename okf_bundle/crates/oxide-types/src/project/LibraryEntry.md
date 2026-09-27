---
okf_version: "0.2"
type: Class
title: LibraryEntry
description: "One library reference recorded in `.snxprj`. The project loader"
resource: crates/oxide-types/src/project.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:27:17Z"
concept_id: crates/oxide-types/src/project/LibraryEntry
language: rust
---

# LibraryEntry

One library reference recorded in `.snxprj`. The project loader

## Signature

```rust
pub struct LibraryEntry
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

One library reference recorded in `.snxprj`. The project loader
iterates this list at open-time and mounts each library via the
matching adapter (currently `LocalGitAdapter` for all three kinds).

See `docs/internal/docs/v0.9-library-plan.md` for the data-model
rationale.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `path`
- `kind`
- `library_id`

## Source
Lines 48–58 in `crates/oxide-types/src/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-types/src/project.md) |
