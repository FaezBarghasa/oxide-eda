---
okf_version: "0.2"
type: Function
title: commit_touches_path
resource: crates/oxide-library/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:59:15Z"
concept_id: crates/oxide-library/src/lib/commit_touches_path
language: rust
---

# commit_touches_path

## Signature

```rust
fn commit_touches_path(
        repo: &git2::Repository,
        commit: &git2::Commit<'_>,
        rel_path: &str,
    ) -> Result<bool, LibraryError>
```

## Source
Lines 366–394 in `crates/oxide-library/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-library/src/lib.md) |
| called_by | [project_file_history](/crates/oxide-library/src/lib/project_file_history.md) |
