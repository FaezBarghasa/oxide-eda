---
okf_version: "0.2"
type: Function
title: insert_footprint
resource: crates/oxide-library-server/src/db/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:44:57Z"
concept_id: crates/oxide-library-server/src/db/mod/insert_footprint
language: rust
---

# insert_footprint

## Signature

```rust
impl AppState { pub fn insert_footprint(&self, library_id: Uuid, fp: &Footprint) -> Result<(), ApiError> }
```

## Visibility

- `pub`

## Source
Lines 333–344 in `crates/oxide-library-server/src/db/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [db](/crates/oxide-library-server/src/db/mod.md) |
| calls | [upsert_primitive](/crates/oxide-library-server/src/db/mod/upsert_primitive.md) |
