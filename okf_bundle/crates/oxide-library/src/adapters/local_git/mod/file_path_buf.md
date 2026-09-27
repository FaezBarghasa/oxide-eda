---
okf_version: "0.2"
type: Function
title: file_path_buf
description: "Borrow the absolute path to the `.snxlib` file itself."
resource: crates/oxide-library/src/adapters/local_git/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/adapters/local_git/mod/file_path_buf
language: rust
---

# file_path_buf

Borrow the absolute path to the `.snxlib` file itself.

## Signature

```rust
impl LocalGitAdapter { pub fn file_path_buf(&self) -> &Path }
```

## Visibility

- `pub`

## Docstring

Borrow the absolute path to the `.snxlib` file itself.

## Source
Lines 338–340 in `crates/oxide-library/src/adapters/local_git/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git](/crates/oxide-library/src/adapters/local_git/mod.md) |
