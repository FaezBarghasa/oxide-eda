---
okf_version: "0.2"
type: Function
title: has_stray_tmp
description: "True if `dir` contains a leftover atomic-write temp sibling"
resource: crates/oxide-types/src/atomic_io.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/atomic_io/has_stray_tmp
language: rust
---

# has_stray_tmp

True if `dir` contains a leftover atomic-write temp sibling

## Signature

```rust
fn has_stray_tmp(dir: &Path) -> bool
```

## Docstring

True if `dir` contains a leftover atomic-write temp sibling
(`*.tmp`). `atomic_write` picks a unique per-writer name (#416),
so tests must scan for any match instead of a fixed name.

## Source
Lines 119–124 in `crates/oxide-types/src/atomic_io.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [atomic_io](/crates/oxide-types/src/atomic_io.md) |
