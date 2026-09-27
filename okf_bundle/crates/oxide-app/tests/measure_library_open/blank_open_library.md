---
okf_version: "0.2"
type: Function
title: blank_open_library
description: "A fresh `OpenLibrary` display entry with empty caches — mirrors what"
resource: crates/oxide-app/tests/measure_library_open.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:51:25Z"
concept_id: crates/oxide-app/tests/measure_library_open/blank_open_library
language: rust
---

# blank_open_library

A fresh `OpenLibrary` display entry with empty caches — mirrors what

## Signature

```rust
fn blank_open_library(snxlib: &Path, adapter: &LocalGitAdapter) -> OpenLibrary
```

## Docstring

A fresh `OpenLibrary` display entry with empty caches — mirrors what
`LibraryState::open_library` builds before it calls `reload_tables`.

## Source
Lines 183–195 in `crates/oxide-app/tests/measure_library_open.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [measure_library_open](/crates/oxide-app/tests/measure_library_open.md) |
| calls | [library_id](/crates/oxide-library/src/adapter/library_id.md) |
| called_by | [measure_scale](/crates/oxide-app/tests/measure_library_open/measure_scale.md) |
