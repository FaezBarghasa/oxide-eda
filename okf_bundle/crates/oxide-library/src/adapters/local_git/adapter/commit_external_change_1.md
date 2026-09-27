---
okf_version: "0.2"
type: Function
title: commit_external_change
resource: crates/oxide-library/src/adapters/local_git/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/local_git/adapter/commit_external_change_1
language: rust
---

# commit_external_change

## Signature

```rust
fn commit_external_change(&self, abs_path: &Path, message: &str) -> Result<(), LibraryError>
```

## Source
Lines 446–460 in `crates/oxide-library/src/adapters/local_git/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapters/local_git/adapter.md) |
