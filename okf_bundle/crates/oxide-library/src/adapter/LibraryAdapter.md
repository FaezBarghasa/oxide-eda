---
okf_version: "0.2"
type: Class
title: LibraryAdapter
description: "Storage backend abstraction. All flavours (LocalGit, Database, Plm)"
resource: crates/oxide-library/src/adapter.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapter/LibraryAdapter
language: rust
---

# LibraryAdapter

Storage backend abstraction. All flavours (LocalGit, Database, Plm)

## Signature

```rust
pub trait LibraryAdapter
```

## Visibility

- `pub`

## Docstring

Storage backend abstraction. All flavours (LocalGit, Database, Plm)
implement this.

**Default impls:** every method has a `LibraryError::Backend("not impl")`
default so individual adapters can override only the surface they
actually support. `LocalGitAdapter` and `DatabaseAdapter` supply
real implementations.

## Source
Lines 142–510 in `crates/oxide-library/src/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapter.md) |
| calls | [library_classes](/crates/oxide-library/src/adapter/library_classes.md) |
| calls | [update_library_classes](/crates/oxide-library/src/adapter/update_library_classes.md) |
