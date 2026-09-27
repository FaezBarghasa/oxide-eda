---
okf_version: "0.2"
type: Function
title: local_git_iter_rows_across_tables
description: "`iter_rows` walks every `[tables.<name>]` inside the `.snxlib` and"
resource: crates/oxide-library/tests/local_git_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:10:08Z"
concept_id: crates/oxide-library/tests/local_git_adapter/local_git_iter_rows_across_tables
language: rust
---

# local_git_iter_rows_across_tables

`iter_rows` walks every `[tables.<name>]` inside the `.snxlib` and

## Signature

```rust
fn local_git_iter_rows_across_tables()
```

## Decorators

- `test`

## Docstring

`iter_rows` walks every `[tables.<name>]` inside the `.snxlib` and
pairs each row with its table name.
[test]

## Source
Lines 785–829 in `crates/oxide-library/tests/local_git_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git_adapter](/crates/oxide-library/tests/local_git_adapter.md) |
| calls | [snxlib_path](/crates/oxide-library/tests/local_git_adapter/snxlib_path.md) |
| calls | [empty_snx_manifest](/crates/oxide-library/tests/local_git_adapter/empty_snx_manifest.md) |
| calls | [fixture_row](/crates/oxide-library/tests/local_git_adapter/fixture_row.md) |
