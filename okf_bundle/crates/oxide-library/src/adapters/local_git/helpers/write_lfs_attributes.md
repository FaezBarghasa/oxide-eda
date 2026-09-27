---
okf_version: "0.2"
type: Function
title: write_lfs_attributes
resource: crates/oxide-library/src/adapters/local_git/helpers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/adapters/local_git/helpers/write_lfs_attributes
language: rust
---

# write_lfs_attributes

## Signature

```rust
pub(super) fn write_lfs_attributes(root_dir: &Path) -> Result<(), LibraryError>
```

## Visibility

- `pub(super)`

## Source
Lines 162–174 in `crates/oxide-library/src/adapters/local_git/helpers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [helpers](/crates/oxide-library/src/adapters/local_git/helpers.md) |
| called_by | [init](/crates/oxide-library/src/adapters/local_git/mod/init.md) |
