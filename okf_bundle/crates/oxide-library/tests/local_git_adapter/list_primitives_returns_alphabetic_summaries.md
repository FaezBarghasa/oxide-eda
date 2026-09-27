---
okf_version: "0.2"
type: Function
title: list_primitives_returns_alphabetic_summaries
description: "`list_symbols` / `list_footprints` / `list_sims` walk the per-kind dir,"
resource: crates/oxide-library/tests/local_git_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:10:08Z"
concept_id: crates/oxide-library/tests/local_git_adapter/list_primitives_returns_alphabetic_summaries
language: rust
---

# list_primitives_returns_alphabetic_summaries

`list_symbols` / `list_footprints` / `list_sims` walk the per-kind dir,

## Signature

```rust
fn list_primitives_returns_alphabetic_summaries()
```

## Decorators

- `test`

## Docstring

`list_symbols` / `list_footprints` / `list_sims` walk the per-kind dir,
return one summary per file, alphabetically sorted by name.
[test]

## Source
Lines 386–410 in `crates/oxide-library/tests/local_git_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git_adapter](/crates/oxide-library/tests/local_git_adapter.md) |
| calls | [init_adapter](/crates/oxide-library/tests/local_git_adapter/init_adapter.md) |
| calls | [fixture_symbol](/crates/oxide-library/tests/local_git_adapter/fixture_symbol.md) |
| calls | [fixture_footprint](/crates/oxide-library/tests/local_git_adapter/fixture_footprint.md) |
