---
okf_version: "0.2"
type: Function
title: fresh_state
resource: crates/oxide-library-server/tests/integration_db.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:44:00Z"
concept_id: crates/oxide-library-server/tests/integration_db/fresh_state
language: rust
---

# fresh_state

## Signature

```rust
fn fresh_state() -> AppState
```

## Source
Lines 62–69 in `crates/oxide-library-server/tests/integration_db.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [integration_db](/crates/oxide-library-server/tests/integration_db.md) |
| calls | [ensure_test_token](/crates/oxide-library-server/tests/integration_db/ensure_test_token.md) |
| called_by | [lock_contention_second_attempt_blocks_until_release](/crates/oxide-library-server/tests/integration_db/lock_contention_second_attempt_blocks_until_release.md) |
| called_by | [lock_contention_ttl_expiry_allows_takeover](/crates/oxide-library-server/tests/integration_db/lock_contention_ttl_expiry_allows_takeover.md) |
| called_by | [locks_endpoint_returns_409_when_held](/crates/oxide-library-server/tests/integration_db/locks_endpoint_returns_409_when_held.md) |
| called_by | [migrations_apply_cleanly](/crates/oxide-library-server/tests/integration_db/migrations_apply_cleanly.md) |
| called_by | [route_delete_row](/crates/oxide-library-server/tests/integration_db/route_delete_row.md) |
| called_by | [route_post_duplicate_row_conflicts_and_preserves_original](/crates/oxide-library-server/tests/integration_db/route_post_duplicate_row_conflicts_and_preserves_original.md) |
| called_by | [route_post_row_then_get](/crates/oxide-library-server/tests/integration_db/route_post_row_then_get.md) |
| called_by | [route_put_row_updates](/crates/oxide-library-server/tests/integration_db/route_put_row_updates.md) |
| called_by | [route_tables_lists_empty](/crates/oxide-library-server/tests/integration_db/route_tables_lists_empty.md) |
| called_by | [route_unauthenticated_returns_401](/crates/oxide-library-server/tests/integration_db/route_unauthenticated_returns_401.md) |
