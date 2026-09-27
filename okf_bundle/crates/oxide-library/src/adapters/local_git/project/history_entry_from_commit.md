---
okf_version: "0.2"
type: Function
title: history_entry_from_commit
resource: crates/oxide-library/src/adapters/local_git/project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-library/src/adapters/local_git/project/history_entry_from_commit
language: rust
---

# history_entry_from_commit

## Signature

```rust
fn history_entry_from_commit(commit: &git2::Commit<'_>, stats: CommitPathStats) -> HistoryEntry
```

## Source
Lines 431–461 in `crates/oxide-library/src/adapters/local_git/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-library/src/adapters/local_git/project.md) |
| called_by | [file_history](/crates/oxide-library/src/adapters/local_git/project/file_history.md) |
