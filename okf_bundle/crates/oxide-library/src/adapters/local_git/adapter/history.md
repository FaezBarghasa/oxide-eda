---
okf_version: "0.2"
type: Function
title: history
resource: crates/oxide-library/src/adapters/local_git/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/local_git/adapter/history
language: rust
---

# history

## Signature

```rust
impl LocalGitAdapter { fn history(&self, primitive_path: &Path) -> Result<Vec<HistoryEntry>, LibraryError> }
```

## Source
Lines 462–537 in `crates/oxide-library/src/adapters/local_git/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapters/local_git/adapter.md) |
