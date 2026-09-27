---
okf_version: "0.2"
type: Function
title: database_round_trip_row
description: "Round-trip: insert → read → update → delete, each call hitting its own"
resource: crates/oxide-library/tests/database_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/tests/database_adapter/database_round_trip_row
language: rust
---

# database_round_trip_row

Round-trip: insert → read → update → delete, each call hitting its own

## Signature

```rust
fn database_round_trip_row()
```

## Decorators

- `test`

## Docstring

Round-trip: insert → read → update → delete, each call hitting its own
mock route. Mirrors the LocalGit test plan (`local_git_adapter.rs`)
per `v0.9-refactor-2-plan.md` §8 step 3.5.
[test]

## Source
Lines 277–352 in `crates/oxide-library/tests/database_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [database_adapter](/crates/oxide-library/tests/database_adapter.md) |
| calls | [mk_row](/crates/oxide-library/tests/database_adapter/mk_row.md) |
| calls | [with_mock_server](/crates/oxide-library/tests/database_adapter/with_mock_server.md) |
| calls | [pin](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/pin.md) |
