---
okf_version: "0.2"
type: Function
title: init_open_round_trip_empty_library
description: "Initialising at a non-existent path writes the `.snxlib` file + makes a"
resource: crates/oxide-library/tests/local_git_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:10:08Z"
concept_id: crates/oxide-library/tests/local_git_adapter/init_open_round_trip_empty_library
language: rust
---

# init_open_round_trip_empty_library

Initialising at a non-existent path writes the `.snxlib` file + makes a

## Signature

```rust
fn init_open_round_trip_empty_library()
```

## Decorators

- `test`

## Docstring

Initialising at a non-existent path writes the `.snxlib` file + makes a
git commit; reopening picks up the same manifest.
[test]

## Source
Lines 77–98 in `crates/oxide-library/tests/local_git_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git_adapter](/crates/oxide-library/tests/local_git_adapter.md) |
| calls | [snxlib_path](/crates/oxide-library/tests/local_git_adapter/snxlib_path.md) |
| calls | [empty_snx_manifest](/crates/oxide-library/tests/local_git_adapter/empty_snx_manifest.md) |
