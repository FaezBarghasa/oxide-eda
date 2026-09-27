---
okf_version: "0.2"
type: Function
title: get_sim
resource: crates/oxide-library/src/adapters/local_git/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/local_git/adapter/get_sim
language: rust
---

# get_sim

## Signature

```rust
impl LocalGitAdapter { fn get_sim(&self, uuid: Uuid) -> Result<SimModel, LibraryError> }
```

## Source
Lines 382–384 in `crates/oxide-library/src/adapters/local_git/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapters/local_git/adapter.md) |
