---
okf_version: "0.2"
type: Function
title: locate_symbol_file
resource: crates/oxide-library/src/adapters/local_git/primitives.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/local_git/primitives/locate_symbol_file
language: rust
---

# locate_symbol_file

## Signature

```rust
impl LocalGitAdapter { fn locate_symbol_file(
        &self,
        uuid: Uuid,
    ) -> Result<Option<(PathBuf, SymbolFile)>, LibraryError> }
```

## Source
Lines 393–403 in `crates/oxide-library/src/adapters/local_git/primitives.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [primitives](/crates/oxide-library/src/adapters/local_git/primitives.md) |
