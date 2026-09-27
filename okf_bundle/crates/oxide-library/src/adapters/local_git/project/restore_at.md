---
okf_version: "0.2"
type: Function
title: restore_at
description: "Restore `rel_path` to the state captured by `commit_oid`."
resource: crates/oxide-library/src/adapters/local_git/project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-library/src/adapters/local_git/project/restore_at
language: rust
---

# restore_at

Restore `rel_path` to the state captured by `commit_oid`.

## Signature

```rust
impl LocalGitProjectAdapter { pub fn restore_at(&self, rel_path: &Path, commit_oid: git2::Oid) -> Result<(), LibraryError> }
```

## Visibility

- `pub`

## Docstring

Restore `rel_path` to the state captured by `commit_oid`.
Atomic write — staging file + rename, working tree stays
consistent if the rename fails.

Does NOT create a new commit — that's the app layer's call
(it'll observe the dirty file on next save and commit through
[`commit_path`] with a "Restore from <sha>" message).

## Source
Lines 275–300 in `crates/oxide-library/src/adapters/local_git/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-library/src/adapters/local_git/project.md) |
