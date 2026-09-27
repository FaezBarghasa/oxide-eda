---
okf_version: "0.2"
type: Function
title: root
description: "Borrow the directory holding the `.snxlib` file (the git working tree)."
resource: crates/oxide-library/src/adapters/local_git/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/adapters/local_git/mod/root
language: rust
---

# root

Borrow the directory holding the `.snxlib` file (the git working tree).

## Signature

```rust
impl LocalGitAdapter { pub fn root(&self) -> &Path }
```

## Visibility

- `pub`

## Docstring

Borrow the directory holding the `.snxlib` file (the git working tree).

## Source
Lines 333–335 in `crates/oxide-library/src/adapters/local_git/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git](/crates/oxide-library/src/adapters/local_git/mod.md) |
