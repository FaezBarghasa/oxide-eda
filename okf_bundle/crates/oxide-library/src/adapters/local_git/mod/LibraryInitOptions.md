---
okf_version: "0.2"
type: Class
title: LibraryInitOptions
description: "Library-create options threaded through [`LocalGitAdapter::init`]."
resource: crates/oxide-library/src/adapters/local_git/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/adapters/local_git/mod/LibraryInitOptions
language: rust
---

# LibraryInitOptions

Library-create options threaded through [`LocalGitAdapter::init`].

## Signature

```rust
pub struct LibraryInitOptions
```

## Decorators

- `derive(Debug, Default, Clone, Copy)`

## Visibility

- `pub`

## Docstring

Library-create options threaded through [`LocalGitAdapter::init`].

`enable_git` defaults to **off** — fresh libraries land on disk as
plain files with no `.git/` directory. Users opt in via the
"Enable version control" checkbox on the New Library Options
modal; flipping it on runs `git init` + records an initial commit
just like the legacy behaviour. LFS only matters when version
control is on (no point in `.gitattributes` without a repo).
[derive(Debug, Default, Clone, Copy)]

## Methods

- `enable_git`
- `use_lfs`

## Source
Lines 68–78 in `crates/oxide-library/src/adapters/local_git/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git](/crates/oxide-library/src/adapters/local_git/mod.md) |
