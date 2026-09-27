---
okf_version: "0.2"
type: Function
title: save_symbol_in_container
resource: crates/oxide-library/src/adapters/local_git/primitives.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/local_git/primitives/save_symbol_in_container
language: rust
---

# save_symbol_in_container

## Signature

```rust
impl LocalGitAdapter { pub(super) fn save_symbol_in_container(
        &self,
        sym: Symbol,
        message: &str,
    ) -> Result<(), LibraryError> }
```

## Visibility

- `pub(super)`

## Source
Lines 340–391 in `crates/oxide-library/src/adapters/local_git/primitives.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [primitives](/crates/oxide-library/src/adapters/local_git/primitives.md) |
