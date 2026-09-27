---
okf_version: "0.2"
type: Function
title: assert_primitive_table
description: "---------- Primitive query helpers ----------------------------------------"
resource: crates/oxide-library-server/src/db/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:44:57Z"
concept_id: crates/oxide-library-server/src/db/mod/assert_primitive_table
language: rust
---

# assert_primitive_table

---------- Primitive query helpers ----------------------------------------

## Signature

```rust
fn assert_primitive_table(table: &'static str)
```

## Docstring

---------- Primitive query helpers ----------------------------------------

## Source
Lines 398–403 in `crates/oxide-library-server/src/db/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [db](/crates/oxide-library-server/src/db/mod.md) |
| called_by | [fetch_primitive_payload](/crates/oxide-library-server/src/db/mod/fetch_primitive_payload.md) |
| called_by | [list_primitive_summaries](/crates/oxide-library-server/src/db/mod/list_primitive_summaries.md) |
| called_by | [upsert_primitive](/crates/oxide-library-server/src/db/mod/upsert_primitive.md) |
