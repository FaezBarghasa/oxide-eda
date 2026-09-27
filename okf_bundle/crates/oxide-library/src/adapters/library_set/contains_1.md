---
okf_version: "0.2"
type: Function
title: contains
description: "True if any mounted adapter exposes the given `library_id`."
resource: crates/oxide-library/src/adapters/library_set.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/library_set/contains_1
language: rust
---

# contains

True if any mounted adapter exposes the given `library_id`.

## Signature

```rust
pub fn contains(&self, library_id: Uuid) -> bool
```

## Visibility

- `pub`

## Docstring

True if any mounted adapter exposes the given `library_id`.

## Source
Lines 149–151 in `crates/oxide-library/src/adapters/library_set.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_set](/crates/oxide-library/src/adapters/library_set.md) |
