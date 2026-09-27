---
okf_version: "0.2"
type: Function
title: fixture_row
resource: crates/oxide-library/src/where_used.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/where_used/fixture_row
language: rust
---

# fixture_row

## Signature

```rust
fn fixture_row(symbol: PrimitiveRef, footprint: Option<PrimitiveRef>) -> ComponentRow
```

## Source
Lines 209–235 in `crates/oxide-library/src/where_used.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [where_used](/crates/oxide-library/src/where_used.md) |
| called_by | [ingest_row_replaces_prior_primitive_links](/crates/oxide-library/src/where_used/ingest_row_replaces_prior_primitive_links.md) |
| called_by | [rebuild_from_rows_replaces_state](/crates/oxide-library/src/where_used/rebuild_from_rows_replaces_state.md) |
| called_by | [rows_for_primitive_returns_referencing_row](/crates/oxide-library/src/where_used/rows_for_primitive_returns_referencing_row.md) |
