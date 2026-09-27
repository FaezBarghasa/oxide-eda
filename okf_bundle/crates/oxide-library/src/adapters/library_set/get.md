---
okf_version: "0.2"
type: Function
title: get
description: "Borrow the first mounted adapter exposing `library_id`, if any."
resource: crates/oxide-library/src/adapters/library_set.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/library_set/get
language: rust
---

# get

Borrow the first mounted adapter exposing `library_id`, if any.

## Signature

```rust
impl LibrarySet { pub fn get(&self, library_id: Uuid) -> Option<&dyn LibraryAdapter> }
```

## Visibility

- `pub`

## Docstring

Borrow the first mounted adapter exposing `library_id`, if any.

## Source
Lines 159–162 in `crates/oxide-library/src/adapters/library_set.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_set](/crates/oxide-library/src/adapters/library_set.md) |
