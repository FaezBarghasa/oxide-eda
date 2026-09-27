---
okf_version: "0.2"
type: Function
title: migrations_apply_cleanly
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
concept_id: crates/oxide-library-server/tests/integration_db/migrations_apply_cleanly
language: rust
---

# migrations_apply_cleanly

[tokio::test]

## Signature

```rust
fn migrations_apply_cleanly()
```

## Decorators

- `tokio::test`

## Docstring

[tokio::test]

## Source
Lines 89–95 in `crates/oxide-library-server/tests/integration_db.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [integration_db](/crates/oxide-library-server/tests/integration_db.md) |
| calls | [fresh_state](/crates/oxide-library-server/tests/integration_db/fresh_state.md) |
