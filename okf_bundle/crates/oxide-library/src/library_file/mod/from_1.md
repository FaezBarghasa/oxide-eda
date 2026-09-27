---
okf_version: "0.2"
type: Function
title: from
description: "Funnel `.snxlib` parse / write failures into the adapter's error"
resource: crates/oxide-library/src/library_file/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/library_file/mod/from_1
language: rust
---

# from

Funnel `.snxlib` parse / write failures into the adapter's error

## Signature

```rust
fn from(value: LibraryFileError) -> Self
```

## Docstring

Funnel `.snxlib` parse / write failures into the adapter's error
channel so callers can `?` through `LibraryAdapter` methods without
matching on the inner enum.

## Source
Lines 303–305 in `crates/oxide-library/src/library_file/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_file](/crates/oxide-library/src/library_file/mod.md) |
