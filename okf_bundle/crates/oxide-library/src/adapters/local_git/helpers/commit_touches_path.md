---
okf_version: "0.2"
type: Function
title: commit_touches_path
description: "True if `commit` modified `rel_path` relative to *any* of its"
resource: crates/oxide-library/src/adapters/local_git/helpers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/adapters/local_git/helpers/commit_touches_path
language: rust
---

# commit_touches_path

True if `commit` modified `rel_path` relative to *any* of its

## Signature

```rust
pub(super) fn commit_touches_path(
    repo: &git2::Repository,
    commit: &git2::Commit<'_>,
    rel_path: &str,
) -> Result<bool, LibraryError>
```

## Visibility

- `pub(super)`

## Docstring

True if `commit` modified `rel_path` relative to *any* of its
parents (or, for the root commit, if the path exists in its
tree). Mirrors the behaviour of `git log -- <path>` for the simple
non-rename case the scaffold targets.

## Source
Lines 42–71 in `crates/oxide-library/src/adapters/local_git/helpers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [helpers](/crates/oxide-library/src/adapters/local_git/helpers.md) |
