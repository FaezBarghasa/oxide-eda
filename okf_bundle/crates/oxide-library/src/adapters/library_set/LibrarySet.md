---
okf_version: "0.2"
type: Class
title: LibrarySet
description: "A bag of mounted libraries, keyed by [`MountKey`]."
resource: crates/oxide-library/src/adapters/library_set.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/library_set/LibrarySet
language: rust
---

# LibrarySet

A bag of mounted libraries, keyed by [`MountKey`].

## Signature

```rust
pub struct LibrarySet
```

## Decorators

- `derive(Default)`

## Visibility

- `pub`

## Docstring

A bag of mounted libraries, keyed by [`MountKey`].

`Default` constructs an empty set; call [`Self::mount`] to add adapters.
[derive(Default)]

## Methods

- `libs`

## Source
Lines 79–81 in `crates/oxide-library/src/adapters/library_set.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_set](/crates/oxide-library/src/adapters/library_set.md) |
