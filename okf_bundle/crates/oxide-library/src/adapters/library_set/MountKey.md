---
okf_version: "0.2"
type: Class
title: MountKey
description: "Mount key for an adapter inside a [`LibrarySet`]."
resource: crates/oxide-library/src/adapters/library_set.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/library_set/MountKey
language: rust
---

# MountKey

Mount key for an adapter inside a [`LibrarySet`].

## Signature

```rust
pub enum MountKey
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Hash)`

## Visibility

- `pub`

## Docstring

Mount key for an adapter inside a [`LibrarySet`].

File-backed adapters (`LocalGitAdapter`) key by their absolute
`.snxlib` file path so the same `library_id` mounted at two
distinct on-disk locations is allowed. Path-less adapters
(`DatabaseAdapter`) fall back to keying by `library_id`.
[derive(Debug, Clone, PartialEq, Eq, Hash)]

## Source
Lines 59–64 in `crates/oxide-library/src/adapters/library_set.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_set](/crates/oxide-library/src/adapters/library_set.md) |
