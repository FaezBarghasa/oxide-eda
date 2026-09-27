---
okf_version: "0.2"
type: Function
title: default_install_path
description: Compute default install path for a dependency relative to the project root.
resource: crates/oxide-library/src/dependency/installer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:29:18Z"
concept_id: crates/oxide-library/src/dependency/installer/default_install_path_1
language: rust
---

# default_install_path

Compute default install path for a dependency relative to the project root.

## Signature

```rust
pub fn default_install_path(name: &str) -> PathBuf
```

## Visibility

- `pub`

## Docstring

Compute default install path for a dependency relative to the project root.

## Source
Lines 18–20 in `crates/oxide-library/src/dependency/installer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [installer](/crates/oxide-library/src/dependency/installer.md) |
