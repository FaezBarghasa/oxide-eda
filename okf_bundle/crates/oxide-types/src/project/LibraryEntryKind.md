---
okf_version: "0.2"
type: Class
title: LibraryEntryKind
description: "How a [`LibraryEntry`] resolves on disk. Project-local libraries live"
resource: crates/oxide-types/src/project.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:27:17Z"
concept_id: crates/oxide-types/src/project/LibraryEntryKind
language: rust
---

# LibraryEntryKind

How a [`LibraryEntry`] resolves on disk. Project-local libraries live

## Signature

```rust
pub enum LibraryEntryKind
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)`
- `serde(rename_all = "snake_case")`

## Visibility

- `pub`

## Docstring

How a [`LibraryEntry`] resolves on disk. Project-local libraries live
under the project directory and use a relative path; shared / global
libraries live elsewhere on the user's machine and use an absolute
path. Drives the auto-mount path resolution at project-open time.
[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
[serde(rename_all = "snake_case")]

## Source
Lines 31–39 in `crates/oxide-types/src/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-types/src/project.md) |
