---
okf_version: "0.2"
type: Function
title: commit_to_history_entry
description: "Project a `git2::Commit` onto the trait-level [`HistoryEntry`]."
resource: crates/oxide-library/src/adapters/local_git/helpers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/adapters/local_git/helpers/commit_to_history_entry
language: rust
---

# commit_to_history_entry

Project a `git2::Commit` onto the trait-level [`HistoryEntry`].

## Signature

```rust
pub(super) fn commit_to_history_entry(commit: &git2::Commit<'_>) -> HistoryEntry
```

## Visibility

- `pub(super)`

## Docstring

Project a `git2::Commit` onto the trait-level [`HistoryEntry`].

Diff-stat fields (`additions`, `deletions`, `files_changed`) stay
at the scaffold defaults — Stage 17 ships the list shape without
the lazy diff plumbing. The author timestamp is preferred over
the committer's so rebases/cherry-picks don't visually skew the
"12 minutes ago" labels.

## Source
Lines 14–36 in `crates/oxide-library/src/adapters/local_git/helpers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [helpers](/crates/oxide-library/src/adapters/local_git/helpers.md) |
