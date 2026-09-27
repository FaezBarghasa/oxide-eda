---
okf_version: "0.2"
type: Function
title: commit_external_change
description: Commit an externally-edited file (user opened the file in a
resource: crates/oxide-library/src/adapters/local_git/project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-library/src/adapters/local_git/project/commit_external_change
language: rust
---

# commit_external_change

Commit an externally-edited file (user opened the file in a

## Signature

```rust
impl LocalGitProjectAdapter { pub fn commit_external_change(
        &self,
        abs_path: &Path,
        message: &str,
    ) -> Result<git2::Oid, LibraryError> }
```

## Visibility

- `pub`

## Docstring

Commit an externally-edited file (user opened the file in a
text editor and saved). Wraps [`commit_path`] with a
reasonable default message when the caller doesn't have
context.

## Source
Lines 165–183 in `crates/oxide-library/src/adapters/local_git/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-library/src/adapters/local_git/project.md) |
