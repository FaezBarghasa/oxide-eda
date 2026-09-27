---
okf_version: "0.2"
type: Function
title: file_name_str
resource: crates/oxide-library/src/adapters/local_git/helpers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/adapters/local_git/helpers/file_name_str
language: rust
---

# file_name_str

## Signature

```rust
pub(super) fn file_name_str(p: &Path) -> Result<String, LibraryError>
```

## Visibility

- `pub(super)`

## Source
Lines 150–160 in `crates/oxide-library/src/adapters/local_git/helpers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [helpers](/crates/oxide-library/src/adapters/local_git/helpers.md) |
| called_by | [init](/crates/oxide-library/src/adapters/local_git/mod/init.md) |
| called_by | [snxlib_rel_path](/crates/oxide-library/src/adapters/local_git/primitives/snxlib_rel_path.md) |
