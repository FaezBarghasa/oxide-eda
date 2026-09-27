---
okf_version: "0.2"
type: Function
title: fixture_row
resource: crates/oxide-library-server/tests/integration_db.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:44:00Z"
concept_id: crates/oxide-library-server/tests/integration_db/fixture_row
language: rust
---

# fixture_row

## Signature

```rust
fn fixture_row(internal_pn: &str) -> ComponentRow
```

## Source
Lines 34–60 in `crates/oxide-library-server/tests/integration_db.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [integration_db](/crates/oxide-library-server/tests/integration_db.md) |
| called_by | [route_delete_row](/crates/oxide-library-server/tests/integration_db/route_delete_row.md) |
| called_by | [route_post_duplicate_row_conflicts_and_preserves_original](/crates/oxide-library-server/tests/integration_db/route_post_duplicate_row_conflicts_and_preserves_original.md) |
| called_by | [route_post_row_then_get](/crates/oxide-library-server/tests/integration_db/route_post_row_then_get.md) |
| called_by | [route_put_row_updates](/crates/oxide-library-server/tests/integration_db/route_put_row_updates.md) |
