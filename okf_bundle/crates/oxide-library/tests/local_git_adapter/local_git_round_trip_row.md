---
okf_version: "0.2"
type: Function
title: local_git_round_trip_row
description: "`insert_row` → `read_row` round-trip; `update_row` mutates in place;"
resource: crates/oxide-library/tests/local_git_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:10:08Z"
concept_id: crates/oxide-library/tests/local_git_adapter/local_git_round_trip_row
language: rust
---

# local_git_round_trip_row

`insert_row` → `read_row` round-trip; `update_row` mutates in place;

## Signature

```rust
fn local_git_round_trip_row()
```

## Decorators

- `test`

## Docstring

`insert_row` → `read_row` round-trip; `update_row` mutates in place;
`delete_row` removes the row and `read_row` then returns `NotFound`.
[test]

## Source
Lines 734–780 in `crates/oxide-library/tests/local_git_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git_adapter](/crates/oxide-library/tests/local_git_adapter.md) |
| calls | [snxlib_path](/crates/oxide-library/tests/local_git_adapter/snxlib_path.md) |
| calls | [empty_snx_manifest](/crates/oxide-library/tests/local_git_adapter/empty_snx_manifest.md) |
| calls | [fixture_row](/crates/oxide-library/tests/local_git_adapter/fixture_row.md) |
