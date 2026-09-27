---
okf_version: "0.2"
type: Function
title: mount
description: Mount a library.
resource: crates/oxide-library/src/adapters/library_set.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/library_set/mount
language: rust
---

# mount

Mount a library.

## Signature

```rust
impl LibrarySet { pub fn mount(&mut self, lib: Box<dyn LibraryAdapter>) -> Result<(), LibraryError> }
```

## Visibility

- `pub`

## Docstring

Mount a library.

Returns `Conflict` when the same mount key is already in use:
duplicate `.snxlib` file path for file-backed adapters, or
duplicate `library_id` for path-less adapters. Two file-backed
adapters with the same `library_id` at *different* paths are
allowed — the Components Panel dedups by id at presentation
time.

Use [`Self::remount`] to replace an existing mount in one step.

## Source
Lines 99–112 in `crates/oxide-library/src/adapters/library_set.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_set](/crates/oxide-library/src/adapters/library_set.md) |
