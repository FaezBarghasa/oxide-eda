---
okf_version: "0.2"
type: Function
title: init_creates_snxlib_file_and_repo
description: "[test]"
resource: crates/oxide-library/src/adapters/local_git/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/adapters/local_git/mod/init_creates_snxlib_file_and_repo
language: rust
---

# init_creates_snxlib_file_and_repo

[test]

## Signature

```rust
fn init_creates_snxlib_file_and_repo()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 381–401 in `crates/oxide-library/src/adapters/local_git/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git](/crates/oxide-library/src/adapters/local_git/mod.md) |
| calls | [fixture_snxlib_path](/crates/oxide-library/src/adapters/local_git/mod/fixture_snxlib_path.md) |
| calls | [fixture_snx_manifest](/crates/oxide-library/src/adapters/local_git/mod/fixture_snx_manifest.md) |
