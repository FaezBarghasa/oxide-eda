---
okf_version: "0.2"
type: Function
title: postgres_migrations_apply_when_env_set
description: "[tokio::test]"
resource: crates/oxide-library-server/tests/integration_db.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:44:00Z"
concept_id: crates/oxide-library-server/tests/integration_db/postgres_migrations_apply_when_env_set
language: rust
---

# postgres_migrations_apply_when_env_set

[tokio::test]

## Signature

```rust
fn postgres_migrations_apply_when_env_set()
```

## Decorators

- `tokio::test`
- `ignore = "requires OXIDE_TEST_PG_URL"`

## Docstring

[tokio::test]
[ignore = "requires OXIDE_TEST_PG_URL"]

## Source
Lines 374–381 in `crates/oxide-library-server/tests/integration_db.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [integration_db](/crates/oxide-library-server/tests/integration_db.md) |
