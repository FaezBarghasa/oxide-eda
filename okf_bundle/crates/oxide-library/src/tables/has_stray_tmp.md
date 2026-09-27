---
okf_version: "0.2"
type: Function
title: has_stray_tmp
description: "True if `dir` contains a leftover atomic-write temp sibling"
resource: crates/oxide-library/src/tables.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/tables/has_stray_tmp
language: rust
---

# has_stray_tmp

True if `dir` contains a leftover atomic-write temp sibling

## Signature

```rust
fn has_stray_tmp(dir: &std::path::Path) -> bool
```

## Docstring

True if `dir` contains a leftover atomic-write temp sibling
(`*.tmp`). `atomic_write` picks a unique per-writer name (#416),
so tests must scan for any match instead of a fixed name.

## Source
Lines 474–479 in `crates/oxide-library/src/tables.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tables](/crates/oxide-library/src/tables.md) |
