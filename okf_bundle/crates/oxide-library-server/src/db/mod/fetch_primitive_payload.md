---
okf_version: "0.2"
type: Function
title: fetch_primitive_payload
resource: crates/oxide-library-server/src/db/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:44:57Z"
concept_id: crates/oxide-library-server/src/db/mod/fetch_primitive_payload
language: rust
---

# fetch_primitive_payload

## Signature

```rust
fn fetch_primitive_payload(
    db: &Surreal<Db>,
    table: &'static str,
    library_id: Uuid,
    uuid: Uuid,
) -> Result<Option<String>, ApiError>
```

## Source
Lines 454–474 in `crates/oxide-library-server/src/db/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [db](/crates/oxide-library-server/src/db/mod.md) |
| calls | [assert_primitive_table](/crates/oxide-library-server/src/db/mod/assert_primitive_table.md) |
| called_by | [fetch_footprint](/crates/oxide-library-server/src/db/mod/fetch_footprint.md) |
| called_by | [fetch_sim](/crates/oxide-library-server/src/db/mod/fetch_sim.md) |
| called_by | [fetch_symbol](/crates/oxide-library-server/src/db/mod/fetch_symbol.md) |
