---
okf_version: "0.2"
type: Function
title: bearer_header
resource: crates/oxide-library-server/tests/integration_db.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:44:00Z"
concept_id: crates/oxide-library-server/tests/integration_db/bearer_header
language: rust
---

# bearer_header

## Signature

```rust
fn bearer_header() -> String
```

## Source
Lines 71–73 in `crates/oxide-library-server/tests/integration_db.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [integration_db](/crates/oxide-library-server/tests/integration_db.md) |
| called_by | [locks_endpoint_returns_409_when_held](/crates/oxide-library-server/tests/integration_db/locks_endpoint_returns_409_when_held.md) |
| called_by | [route_delete_row](/crates/oxide-library-server/tests/integration_db/route_delete_row.md) |
| called_by | [route_post_duplicate_row_conflicts_and_preserves_original](/crates/oxide-library-server/tests/integration_db/route_post_duplicate_row_conflicts_and_preserves_original.md) |
| called_by | [route_post_row_then_get](/crates/oxide-library-server/tests/integration_db/route_post_row_then_get.md) |
| called_by | [route_put_row_updates](/crates/oxide-library-server/tests/integration_db/route_put_row_updates.md) |
| called_by | [route_tables_lists_empty](/crates/oxide-library-server/tests/integration_db/route_tables_lists_empty.md) |
