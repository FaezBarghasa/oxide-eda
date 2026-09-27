---
okf_version: "0.2"
type: Function
title: update_library_classes
resource: crates/oxide-library/src/adapters/local_git/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/local_git/adapter/update_library_classes
language: rust
---

# update_library_classes

## Signature

```rust
impl LocalGitAdapter { fn update_library_classes(
        &self,
        classes: Vec<crate::library_file::ClassEntry>,
        msg: &str,
    ) -> Result<(), LibraryError> }
```

## Source
Lines 120–133 in `crates/oxide-library/src/adapters/local_git/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapters/local_git/adapter.md) |
