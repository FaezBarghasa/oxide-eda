---
okf_version: "0.2"
type: Function
title: fixture_symbol
description: ── Primitive CRUD over HTTP ─────────────────────────────────────────────
resource: crates/oxide-library/tests/database_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/tests/database_adapter/fixture_symbol
language: rust
---

# fixture_symbol

── Primitive CRUD over HTTP ─────────────────────────────────────────────

## Signature

```rust
fn fixture_symbol() -> Symbol
```

## Docstring

── Primitive CRUD over HTTP ─────────────────────────────────────────────

## Source
Lines 62–69 in `crates/oxide-library/tests/database_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [database_adapter](/crates/oxide-library/tests/database_adapter.md) |
| called_by | [get_symbol_round_trips_through_get_symbols_uuid](/crates/oxide-library/tests/database_adapter/get_symbol_round_trips_through_get_symbols_uuid.md) |
| called_by | [save_symbol_posts_to_symbols_with_message_header](/crates/oxide-library/tests/database_adapter/save_symbol_posts_to_symbols_with_message_header.md) |
