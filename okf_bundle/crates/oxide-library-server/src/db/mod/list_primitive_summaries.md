---
okf_version: "0.2"
type: Function
title: list_primitive_summaries
resource: crates/oxide-library-server/src/db/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:44:57Z"
concept_id: crates/oxide-library-server/src/db/mod/list_primitive_summaries
language: rust
---

# list_primitive_summaries

## Signature

```rust
fn list_primitive_summaries(
    db: &Surreal<Db>,
    table: &'static str,
    library_id: Option<Uuid>,
) -> Result<Vec<PrimitiveSummary>, ApiError>
```

## Source
Lines 476–515 in `crates/oxide-library-server/src/db/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [db](/crates/oxide-library-server/src/db/mod.md) |
| calls | [assert_primitive_table](/crates/oxide-library-server/src/db/mod/assert_primitive_table.md) |
| called_by | [list_footprints](/crates/oxide-library-server/src/db/mod/list_footprints.md) |
| called_by | [list_sims](/crates/oxide-library-server/src/db/mod/list_sims.md) |
| called_by | [list_symbols](/crates/oxide-library-server/src/db/mod/list_symbols.md) |
