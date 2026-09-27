---
okf_version: "0.2"
type: Class
title: ClassEntry
description: "One row of the per-library class registry. `key` is the canonical"
resource: crates/oxide-library/src/library_file/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/library_file/mod/ClassEntry
language: rust
---

# ClassEntry

One row of the per-library class registry. `key` is the canonical

## Signature

```rust
pub struct ClassEntry
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

One row of the per-library class registry. `key` is the canonical
machine identifier stored on `ComponentRow.class`; `label` is the
human-readable name surfaced in pickers.
[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `key`
- `label`

## Source
Lines 85–88 in `crates/oxide-library/src/library_file/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_file](/crates/oxide-library/src/library_file/mod.md) |
