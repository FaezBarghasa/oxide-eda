---
okf_version: "0.2"
type: Class
title: CommitPathStats
description: "Diff-stat summary for one commit's touch on a single file."
resource: crates/oxide-library/src/adapters/local_git/project.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-library/src/adapters/local_git/project/CommitPathStats
language: rust
---

# CommitPathStats

Diff-stat summary for one commit's touch on a single file.

## Signature

```rust
struct CommitPathStats
```

## Docstring

Diff-stat summary for one commit's touch on a single file.
`additions` / `deletions` are the line counts ; `files_changed`
records the path so the History panel can render it.

## Methods

- `files_changed`
- `additions`
- `deletions`

## Source
Lines 372–376 in `crates/oxide-library/src/adapters/local_git/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-library/src/adapters/local_git/project.md) |
