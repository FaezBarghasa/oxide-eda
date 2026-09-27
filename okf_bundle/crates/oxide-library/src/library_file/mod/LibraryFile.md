---
okf_version: "0.2"
type: Class
title: LibraryFile
description: "Top-level on-disk shape of a `.snxlib` file."
resource: crates/oxide-library/src/library_file/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/library_file/mod/LibraryFile
language: rust
---

# LibraryFile

Top-level on-disk shape of a `.snxlib` file.

## Signature

```rust
pub struct LibraryFile
```

## Decorators

- `derive(Debug, Clone, PartialEq)`

## Visibility

- `pub`

## Docstring

Top-level on-disk shape of a `.snxlib` file.

Construct via [`LibraryFile::parse`]; emit via [`LibraryFile::write`].
The two operations form a round-trip: `parse(write(x))` returns a
value equal to `x`.
[derive(Debug, Clone, PartialEq)]

## Methods

- `manifest`
- `tables`

## Source
Lines 44–51 in `crates/oxide-library/src/library_file/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_file](/crates/oxide-library/src/library_file/mod.md) |
