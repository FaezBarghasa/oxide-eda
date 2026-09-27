---
okf_version: "0.2"
type: Class
title: FakeAdapter
description: Minimal in-memory adapter used to exercise resolver mechanics
resource: crates/oxide-library/src/adapters/library_set.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/library_set/FakeAdapter
language: rust
---

# FakeAdapter

Minimal in-memory adapter used to exercise resolver mechanics

## Signature

```rust
struct FakeAdapter
```

## Docstring

Minimal in-memory adapter used to exercise resolver mechanics
without requiring the `local-git` feature. Optionally carries a
fake `library_file_path()` so tests can drive the path-keyed
branch of `MountKey`.

## Methods

- `manifest`
- `symbols`
- `path`
- `read_failure`

## Source
Lines 352–360 in `crates/oxide-library/src/adapters/library_set.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_set](/crates/oxide-library/src/adapters/library_set.md) |
