---
okf_version: "0.2"
type: Function
title: primitive_saves_each_create_a_commit
description: "Each `save_*` produces its own commit (so history mirrors edits)."
resource: crates/oxide-library/tests/local_git_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:10:08Z"
concept_id: crates/oxide-library/tests/local_git_adapter/primitive_saves_each_create_a_commit
language: rust
---

# primitive_saves_each_create_a_commit

Each `save_*` produces its own commit (so history mirrors edits).

## Signature

```rust
fn primitive_saves_each_create_a_commit()
```

## Decorators

- `test`

## Docstring

Each `save_*` produces its own commit (so history mirrors edits).
[test]

## Source
Lines 306–322 in `crates/oxide-library/tests/local_git_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git_adapter](/crates/oxide-library/tests/local_git_adapter.md) |
| calls | [init_adapter](/crates/oxide-library/tests/local_git_adapter/init_adapter.md) |
| calls | [fixture_symbol](/crates/oxide-library/tests/local_git_adapter/fixture_symbol.md) |
| calls | [fixture_footprint](/crates/oxide-library/tests/local_git_adapter/fixture_footprint.md) |
| calls | [fixture_sim](/crates/oxide-library/tests/local_git_adapter/fixture_sim.md) |
