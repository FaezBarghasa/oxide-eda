---
okf_version: "0.2"
type: Function
title: get_sim
resource: crates/oxide-library/src/adapters/database.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/database/get_sim
language: rust
---

# get_sim

## Signature

```rust
impl DatabaseAdapter { fn get_sim(&self, uuid: Uuid) -> Result<SimModel, LibraryError> }
```

## Source
Lines 483–485 in `crates/oxide-library/src/adapters/database.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [database](/crates/oxide-library/src/adapters/database.md) |
