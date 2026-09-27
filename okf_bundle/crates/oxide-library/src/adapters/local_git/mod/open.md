---
okf_version: "0.2"
type: Function
title: open
description: "Open an existing library by `.snxlib` file path. The file's"
resource: crates/oxide-library/src/adapters/local_git/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/adapters/local_git/mod/open
language: rust
---

# open

Open an existing library by `.snxlib` file path. The file's

## Signature

```rust
impl LocalGitAdapter { pub fn open(file_path: impl AsRef<Path>) -> Result<Self, LibraryError> }
```

## Visibility

- `pub`

## Docstring

Open an existing library by `.snxlib` file path. The file's
parent directory must already host a `.git/` — recovery from
a deleted git repo lives in [`Self::recover_init`].

## Source
Lines 203–234 in `crates/oxide-library/src/adapters/local_git/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git](/crates/oxide-library/src/adapters/local_git/mod.md) |
| calls | [parent_dir](/crates/oxide-library/src/adapters/local_git/helpers/parent_dir.md) |
| calls | [synthesize_manifest](/crates/oxide-library/src/adapters/local_git/helpers/synthesize_manifest.md) |
