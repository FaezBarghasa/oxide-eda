---
okf_version: "0.2"
type: Function
title: main
description: "! Build script for `oxide-library`."
resource: crates/oxide-library/build.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/build/main
language: rust
---

# main

! Build script for `oxide-library`.

## Signature

```rust
fn main()
```

## Docstring

! Build script for `oxide-library`.
!
! libgit2-sys 0.17 (transitively pulled by `git2 = "0.19"` under the
! `local-git` feature) calls `OpenProcessToken`, `CryptGenRandom`,
! `RegOpenKeyExW`, etc. but neglects to link `advapi32` itself, so the
! Windows MSVC linker fails when building any binary target (tests,
! examples, dependent bins). Emit the link hint here so the feature
! works out of the box on Windows.

## Source
Lines 10–18 in `crates/oxide-library/build.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [build](/crates/oxide-library/build.md) |
