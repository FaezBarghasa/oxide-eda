---
okf_version: "0.2"
type: Function
title: corrupt_symbol_file_resolves_to_an_error_not_a_missing_uuid
description: "A corrupt `.snxsym` on disk must not read as \"this row's symbol_ref"
resource: crates/oxide-library/tests/local_git_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:10:08Z"
concept_id: crates/oxide-library/tests/local_git_adapter/corrupt_symbol_file_resolves_to_an_error_not_a_missing_uuid
language: rust
---

# corrupt_symbol_file_resolves_to_an_error_not_a_missing_uuid

A corrupt `.snxsym` on disk must not read as "this row's symbol_ref

## Signature

```rust
fn corrupt_symbol_file_resolves_to_an_error_not_a_missing_uuid()
```

## Decorators

- `test`

## Docstring

A corrupt `.snxsym` on disk must not read as "this row's symbol_ref
points at a UUID that isn't in the library". The resolver hands the
caller the parse error so the UI can say what actually happened.
[test]

## Source
Lines 645–695 in `crates/oxide-library/tests/local_git_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git_adapter](/crates/oxide-library/tests/local_git_adapter.md) |
| calls | [snxlib_path](/crates/oxide-library/tests/local_git_adapter/snxlib_path.md) |
| calls | [empty_snx_manifest](/crates/oxide-library/tests/local_git_adapter/empty_snx_manifest.md) |
| calls | [fixture_symbol](/crates/oxide-library/tests/local_git_adapter/fixture_symbol.md) |
