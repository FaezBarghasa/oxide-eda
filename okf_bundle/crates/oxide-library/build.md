---
okf_version: "0.2"
type: Module
title: build
description: "Build script for `oxide-library`."
resource: crates/oxide-library/build.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/build
language: rust
---

# build

Build script for `oxide-library`.

## Docstring

Build script for `oxide-library`.

libgit2-sys 0.17 (transitively pulled by `git2 = "0.19"` under the
`local-git` feature) calls `OpenProcessToken`, `CryptGenRandom`,
`RegOpenKeyExW`, etc. but neglects to link `advapi32` itself, so the
Windows MSVC linker fails when building any binary target (tests,
examples, dependent bins). Emit the link hint here so the feature
works out of the box on Windows.

## Relationships

| Type | Target |
|------|--------|
| related | [main](/crates/oxide-library/build/main.md) |
