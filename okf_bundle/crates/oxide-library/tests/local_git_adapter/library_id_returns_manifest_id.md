---
okf_version: "0.2"
type: Function
title: library_id_returns_manifest_id
description: "`library_id()` reflects the manifest's stable UUID so the resolver can key"
resource: crates/oxide-library/tests/local_git_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:10:08Z"
concept_id: crates/oxide-library/tests/local_git_adapter/library_id_returns_manifest_id
language: rust
---

# library_id_returns_manifest_id

`library_id()` reflects the manifest's stable UUID so the resolver can key

## Signature

```rust
fn library_id_returns_manifest_id()
```

## Decorators

- `test`

## Docstring

`library_id()` reflects the manifest's stable UUID so the resolver can key
`LibrarySet` mounts off the adapter directly.
[test]

## Source
Lines 208–223 in `crates/oxide-library/tests/local_git_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git_adapter](/crates/oxide-library/tests/local_git_adapter.md) |
| calls | [snxlib_path](/crates/oxide-library/tests/local_git_adapter/snxlib_path.md) |
| calls | [empty_snx_manifest](/crates/oxide-library/tests/local_git_adapter/empty_snx_manifest.md) |
