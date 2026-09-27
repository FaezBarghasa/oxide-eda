---
okf_version: "0.2"
type: Function
title: history_returns_per_primitive_commits
description: "Stage 17: `history(primitive_path)` returns one [`HistoryEntry`]"
resource: crates/oxide-library/tests/local_git_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:10:08Z"
concept_id: crates/oxide-library/tests/local_git_adapter/history_returns_per_primitive_commits
language: rust
---

# history_returns_per_primitive_commits

Stage 17: `history(primitive_path)` returns one [`HistoryEntry`]

## Signature

```rust
fn history_returns_per_primitive_commits()
```

## Decorators

- `test`

## Docstring

Stage 17: `history(primitive_path)` returns one [`HistoryEntry`]
per commit that touched the file, newest-first, capped at 50.

Two saves of the same symbol uuid land in the same `.snxsym`
container (`save_symbol_in_container` upserts in place), so the
per-file history must surface both saves' commit messages and
none of the unrelated ones (init commit, footprint commit).
[test]

## Source
Lines 332–381 in `crates/oxide-library/tests/local_git_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git_adapter](/crates/oxide-library/tests/local_git_adapter.md) |
| calls | [init_adapter](/crates/oxide-library/tests/local_git_adapter/init_adapter.md) |
| calls | [fixture_symbol](/crates/oxide-library/tests/local_git_adapter/fixture_symbol.md) |
| calls | [fixture_footprint](/crates/oxide-library/tests/local_git_adapter/fixture_footprint.md) |
