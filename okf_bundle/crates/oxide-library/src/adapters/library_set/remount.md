---
okf_version: "0.2"
type: Function
title: remount
description: "Replace whatever adapter is mounted at `lib`'s mount key with"
resource: crates/oxide-library/src/adapters/library_set.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/library_set/remount
language: rust
---

# remount

Replace whatever adapter is mounted at `lib`'s mount key with

## Signature

```rust
impl LibrarySet { pub fn remount(&mut self, lib: Box<dyn LibraryAdapter>) -> Option<Box<dyn LibraryAdapter>> }
```

## Visibility

- `pub`

## Docstring

Replace whatever adapter is mounted at `lib`'s mount key with
`lib`. Drops the previous adapter and returns it (or `None` if
no previous mount existed).

## Source
Lines 117–120 in `crates/oxide-library/src/adapters/library_set.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_set](/crates/oxide-library/src/adapters/library_set.md) |
