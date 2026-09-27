---
okf_version: "0.2"
type: Function
title: identity_for_repo
resource: crates/oxide-library/src/adapters/local_git/helpers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/adapters/local_git/helpers/identity_for_repo
language: rust
---

# identity_for_repo

## Signature

```rust
pub(super) fn identity_for_repo(repo: &git2::Repository) -> (String, String)
```

## Visibility

- `pub(super)`

## Source
Lines 258–269 in `crates/oxide-library/src/adapters/local_git/helpers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [helpers](/crates/oxide-library/src/adapters/local_git/helpers.md) |
