---
okf_version: "0.2"
type: Module
title: integration_db
description: "Integration tests covering DB schema migrations + the `/tables`"
resource: crates/oxide-library-server/tests/integration_db.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:44:00Z"
concept_id: crates/oxide-library-server/tests/integration_db
language: rust
---

# integration_db

Integration tests covering DB schema migrations + the `/tables`

## Docstring

Integration tests covering DB schema migrations + the `/tables`
and `/rows` HTTP routes for the DBLib row model.

Default backend: in-memory SQLite. Postgres path is gated behind
`OXIDE_TEST_PG_URL` env var so CI without Postgres still passes.

## Relationships

| Type | Target |
|------|--------|
| related | [ensure_test_token](/crates/oxide-library-server/tests/integration_db/ensure_test_token.md) |
| related | [fixture_row](/crates/oxide-library-server/tests/integration_db/fixture_row.md) |
| related | [fresh_state](/crates/oxide-library-server/tests/integration_db/fresh_state.md) |
| related | [bearer_header](/crates/oxide-library-server/tests/integration_db/bearer_header.md) |
| related | [migrations_apply_cleanly](/crates/oxide-library-server/tests/integration_db/migrations_apply_cleanly.md) |
| related | [route_tables_lists_empty](/crates/oxide-library-server/tests/integration_db/route_tables_lists_empty.md) |
| related | [route_post_row_then_get](/crates/oxide-library-server/tests/integration_db/route_post_row_then_get.md) |
| related | [route_post_duplicate_row_conflicts_and_preserves_original](/crates/oxide-library-server/tests/integration_db/route_post_duplicate_row_conflicts_and_preserves_original.md) |
| related | [route_put_row_updates](/crates/oxide-library-server/tests/integration_db/route_put_row_updates.md) |
| related | [route_delete_row](/crates/oxide-library-server/tests/integration_db/route_delete_row.md) |
| related | [route_unauthenticated_returns_401](/crates/oxide-library-server/tests/integration_db/route_unauthenticated_returns_401.md) |
| related | [lock_contention_second_attempt_blocks_until_release](/crates/oxide-library-server/tests/integration_db/lock_contention_second_attempt_blocks_until_release.md) |
| related | [lock_contention_ttl_expiry_allows_takeover](/crates/oxide-library-server/tests/integration_db/lock_contention_ttl_expiry_allows_takeover.md) |
| related | [locks_endpoint_returns_409_when_held](/crates/oxide-library-server/tests/integration_db/locks_endpoint_returns_409_when_held.md) |
| related | [postgres_migrations_apply_when_env_set](/crates/oxide-library-server/tests/integration_db/postgres_migrations_apply_when_env_set.md) |
| related | [chrono](/_dependencies/cargo/chrono.md) |
