---
okf_version: "0.2"
type: Function
title: fresh_symbol_file_path
resource: crates/oxide-library/src/adapters/local_git/primitives.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/local_git/primitives/fresh_symbol_file_path
language: rust
---

# fresh_symbol_file_path

## Signature

```rust
impl LocalGitAdapter { fn fresh_symbol_file_path(
        &self,
        dir: &Path,
        file: &SymbolFile,
    ) -> Result<PathBuf, LibraryError> }
```

## Source
Lines 405–424 in `crates/oxide-library/src/adapters/local_git/primitives.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [primitives](/crates/oxide-library/src/adapters/local_git/primitives.md) |
| calls | [slugify](/crates/oxide-library/src/adapters/local_git/helpers/slugify.md) |
