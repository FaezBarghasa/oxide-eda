---
okf_version: "0.2"
type: Function
title: resolve_library_path
description: "Resolve a [`LibraryEntry`]'s `path` to an absolute path. Project-"
resource: crates/oxide-types/src/project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:27:17Z"
concept_id: crates/oxide-types/src/project/resolve_library_path_1
language: rust
---

# resolve_library_path

Resolve a [`LibraryEntry`]'s `path` to an absolute path. Project-

## Signature

```rust
pub fn resolve_library_path(&self, entry: &LibraryEntry) -> PathBuf
```

## Visibility

- `pub`

## Docstring

Resolve a [`LibraryEntry`]'s `path` to an absolute path. Project-
local entries are joined against `dir`; shared/global entries
are returned as-is. Used by both the auto-mount loop and the
project-tree renderer.

## Source
Lines 236–241 in `crates/oxide-types/src/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-types/src/project.md) |
