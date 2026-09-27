---
okf_version: "0.2"
type: Function
title: library_set_resolves_across_two_local_libs
description: LibrarySet integration test — mount two LocalGit libraries and resolve a
resource: crates/oxide-library/tests/local_git_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:10:08Z"
concept_id: crates/oxide-library/tests/local_git_adapter/library_set_resolves_across_two_local_libs
language: rust
---

# library_set_resolves_across_two_local_libs

LibrarySet integration test — mount two LocalGit libraries and resolve a

## Signature

```rust
fn library_set_resolves_across_two_local_libs()
```

## Decorators

- `test`

## Docstring

LibrarySet integration test — mount two LocalGit libraries and resolve a
`PrimitiveRef` whose `library_id` points at one specific lib. Verifies the
cross-library resolver picks the correct adapter and surfaces unresolved
refs cleanly.
[test]

## Source
Lines 547–639 in `crates/oxide-library/tests/local_git_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git_adapter](/crates/oxide-library/tests/local_git_adapter.md) |
| calls | [snxlib_path](/crates/oxide-library/tests/local_git_adapter/snxlib_path.md) |
| calls | [empty_snx_manifest](/crates/oxide-library/tests/local_git_adapter/empty_snx_manifest.md) |
| calls | [fixture_symbol](/crates/oxide-library/tests/local_git_adapter/fixture_symbol.md) |
| calls | [fixture_footprint](/crates/oxide-library/tests/local_git_adapter/fixture_footprint.md) |
