---
okf_version: "0.2"
type: Function
title: file_history
description: "Per-file commit history newest-first. Returns up to `limit`"
resource: crates/oxide-library/src/adapters/local_git/project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-library/src/adapters/local_git/project/file_history_1
language: rust
---

# file_history

Per-file commit history newest-first. Returns up to `limit`

## Signature

```rust
pub fn file_history(
        &self,
        rel_path: &Path,
        limit: usize,
    ) -> Result<Vec<HistoryEntry>, LibraryError>
```

## Visibility

- `pub`

## Docstring

Per-file commit history newest-first. Returns up to `limit`
entries. Walks the repo's commit graph filtering on commits
whose tree differs from at least one parent at `rel_path`.
Equivalent to `git log -- <rel_path>` semantics.

Empty when no `.git/` exists, the path has never been
committed, or HEAD is unborn (fresh `git init` before any
commit).

## Source
Lines 193–266 in `crates/oxide-library/src/adapters/local_git/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-library/src/adapters/local_git/project.md) |
| calls | [commit_diff_stats_for_path](/crates/oxide-library/src/adapters/local_git/project/commit_diff_stats_for_path.md) |
| calls | [history_entry_from_commit](/crates/oxide-library/src/adapters/local_git/project/history_entry_from_commit.md) |
