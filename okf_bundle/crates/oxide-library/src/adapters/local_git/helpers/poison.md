---
okf_version: "0.2"
type: Function
title: poison
resource: crates/oxide-library/src/adapters/local_git/helpers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/adapters/local_git/helpers/poison
language: rust
---

# poison

## Signature

```rust
fn poison(lock: &RwLock<LibraryFile>)
```

## Source
Lines 302–309 in `crates/oxide-library/src/adapters/local_git/helpers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [helpers](/crates/oxide-library/src/adapters/local_git/helpers.md) |
| called_by | [a_poisoned_lock_does_not_read_as_an_empty_class_registry](/crates/oxide-library/src/adapters/local_git/helpers/a_poisoned_lock_does_not_read_as_an_empty_class_registry.md) |
