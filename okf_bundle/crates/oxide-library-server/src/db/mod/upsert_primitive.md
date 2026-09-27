---
okf_version: "0.2"
type: Function
title: upsert_primitive
resource: crates/oxide-library-server/src/db/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:44:57Z"
concept_id: crates/oxide-library-server/src/db/mod/upsert_primitive
language: rust
---

# upsert_primitive

## Signature

```rust
fn upsert_primitive(
    db: &Surreal<Db>,
    table: &'static str,
    library_id: Uuid,
    uuid: Uuid,
    name: &str,
    payload: &str,
) -> Result<(), ApiError>
```

## Source
Lines 405–452 in `crates/oxide-library-server/src/db/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [db](/crates/oxide-library-server/src/db/mod.md) |
| calls | [assert_primitive_table](/crates/oxide-library-server/src/db/mod/assert_primitive_table.md) |
| called_by | [insert_footprint](/crates/oxide-library-server/src/db/mod/insert_footprint.md) |
| called_by | [insert_sim](/crates/oxide-library-server/src/db/mod/insert_sim.md) |
| called_by | [insert_symbol](/crates/oxide-library-server/src/db/mod/insert_symbol.md) |
