---
okf_version: "0.2"
type: Function
title: library_file_path
description: "Absolute path to the `.snxlib` file itself. `None` for non-file"
resource: crates/oxide-library/src/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapter/library_file_path
language: rust
---

# library_file_path

Absolute path to the `.snxlib` file itself. `None` for non-file

## Signature

```rust
fn library_file_path(&self) -> Option<&std::path::Path>
```

## Docstring

Absolute path to the `.snxlib` file itself. `None` for non-file
adapters.

## Source
Lines 182–184 in `crates/oxide-library/src/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapter.md) |
