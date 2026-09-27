---
okf_version: "0.2"
type: Function
title: save_then_get_symbol_round_trip
description: Save a Symbol → reopen → get_symbol → bytes are identical.
resource: crates/oxide-library/tests/local_git_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:10:08Z"
concept_id: crates/oxide-library/tests/local_git_adapter/save_then_get_symbol_round_trip
language: rust
---

# save_then_get_symbol_round_trip

Save a Symbol → reopen → get_symbol → bytes are identical.

## Signature

```rust
fn save_then_get_symbol_round_trip()
```

## Decorators

- `test`

## Docstring

Save a Symbol → reopen → get_symbol → bytes are identical.
Multi-symbol containers (v0.9 phase 2): the adapter writes the
symbol into a `SymbolFile` TOML envelope named after the
symbol's slugified name (`opamp-dual-8.snxsym`), not
`<uuid>.snxsym`. TOML-only since v0.18.4 (alpha policy, no legacy
JSON support).
[test]

## Source
Lines 232–252 in `crates/oxide-library/tests/local_git_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git_adapter](/crates/oxide-library/tests/local_git_adapter.md) |
| calls | [init_adapter](/crates/oxide-library/tests/local_git_adapter/init_adapter.md) |
| calls | [fixture_symbol](/crates/oxide-library/tests/local_git_adapter/fixture_symbol.md) |
