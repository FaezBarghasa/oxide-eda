---
okf_version: "0.2"
type: Function
title: has_stray_tmp
description: "True if `dir` contains a leftover atomic-write temp sibling (`*.tmp`)."
resource: crates/oxide-app/src/test_support.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/test_support/has_stray_tmp
language: rust
---

# has_stray_tmp

True if `dir` contains a leftover atomic-write temp sibling (`*.tmp`).

## Signature

```rust
pub fn has_stray_tmp(dir: &Path) -> bool
```

## Visibility

- `pub`

## Docstring

True if `dir` contains a leftover atomic-write temp sibling (`*.tmp`).
`atomic_write` picks a unique per-writer name, so callers must scan
for any match rather than a fixed name.

## Source
Lines 101–107 in `crates/oxide-app/src/test_support.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [test_support](/crates/oxide-app/src/test_support.md) |
| calls | [flatten](/crates/oxide-app/src/library/editor/symbol/context_menu/mod/flatten.md) |
