---
okf_version: "0.2"
type: Function
title: commit_to_history_entry
description: Local helpers — duplicated from local_git.rs deliberately so
resource: crates/oxide-library/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:59:15Z"
concept_id: crates/oxide-library/src/lib/commit_to_history_entry
language: rust
---

# commit_to_history_entry

Local helpers — duplicated from local_git.rs deliberately so

## Signature

```rust
fn commit_to_history_entry(commit: &git2::Commit<'_>) -> HistoryEntry
```

## Docstring

Local helpers — duplicated from local_git.rs deliberately so
this routine doesn't depend on adapter internals (the adapter
version is library-rooted; this one walks any repo).

## Source
Lines 342–364 in `crates/oxide-library/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-library/src/lib.md) |
| called_by | [project_file_history](/crates/oxide-library/src/lib/project_file_history.md) |
