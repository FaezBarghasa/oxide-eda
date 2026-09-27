---
okf_version: "0.2"
type: Function
title: get_footprint
description: "The trait defaults report `Backend(\"not implemented\")`, which"
resource: crates/oxide-library/src/adapters/library_set.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/library_set/get_footprint_1
language: rust
---

# get_footprint

The trait defaults report `Backend("not implemented")`, which

## Signature

```rust
fn get_footprint(&self, uuid: Uuid) -> Result<Footprint, LibraryError>
```

## Docstring

The trait defaults report `Backend("not implemented")`, which
the resolver correctly reads as "could not tell". Real
adapters implement all three, so model that here rather than
making every test ref look undecidable.

## Source
Lines 427–432 in `crates/oxide-library/src/adapters/library_set.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_set](/crates/oxide-library/src/adapters/library_set.md) |
