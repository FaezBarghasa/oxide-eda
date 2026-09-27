---
okf_version: "0.2"
type: Function
title: get_footprint
resource: crates/oxide-library/src/adapters/database.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/database/get_footprint
language: rust
---

# get_footprint

## Signature

```rust
impl DatabaseAdapter { fn get_footprint(&self, uuid: Uuid) -> Result<Footprint, LibraryError> }
```

## Source
Lines 479–481 in `crates/oxide-library/src/adapters/database.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [database](/crates/oxide-library/src/adapters/database.md) |
