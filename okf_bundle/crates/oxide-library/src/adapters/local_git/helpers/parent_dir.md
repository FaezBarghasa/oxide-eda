---
okf_version: "0.2"
type: Function
title: parent_dir
resource: crates/oxide-library/src/adapters/local_git/helpers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/adapters/local_git/helpers/parent_dir
language: rust
---

# parent_dir

## Signature

```rust
pub(super) fn parent_dir(p: &Path) -> Result<PathBuf, LibraryError>
```

## Visibility

- `pub(super)`

## Source
Lines 141–148 in `crates/oxide-library/src/adapters/local_git/helpers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [helpers](/crates/oxide-library/src/adapters/local_git/helpers.md) |
| called_by | [init](/crates/oxide-library/src/adapters/local_git/mod/init.md) |
| called_by | [open](/crates/oxide-library/src/adapters/local_git/mod/open.md) |
| called_by | [recover_init](/crates/oxide-library/src/adapters/local_git/mod/recover_init.md) |
