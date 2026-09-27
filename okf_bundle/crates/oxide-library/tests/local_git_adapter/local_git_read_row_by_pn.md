---
okf_version: "0.2"
type: Function
title: local_git_read_row_by_pn
description: "`read_row_by_pn` finds the first matching row across every table."
resource: crates/oxide-library/tests/local_git_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:10:08Z"
concept_id: crates/oxide-library/tests/local_git_adapter/local_git_read_row_by_pn
language: rust
---

# local_git_read_row_by_pn

`read_row_by_pn` finds the first matching row across every table.

## Signature

```rust
fn local_git_read_row_by_pn()
```

## Decorators

- `test`

## Docstring

`read_row_by_pn` finds the first matching row across every table.
[test]

## Source
Lines 833–864 in `crates/oxide-library/tests/local_git_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git_adapter](/crates/oxide-library/tests/local_git_adapter.md) |
| calls | [snxlib_path](/crates/oxide-library/tests/local_git_adapter/snxlib_path.md) |
| calls | [empty_snx_manifest](/crates/oxide-library/tests/local_git_adapter/empty_snx_manifest.md) |
| calls | [fixture_row](/crates/oxide-library/tests/local_git_adapter/fixture_row.md) |
