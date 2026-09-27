---
okf_version: "0.2"
type: Function
title: init_refuses_existing_library
description: "Re-init over an existing `.snxlib` must not silently nuke history."
resource: crates/oxide-library/tests/local_git_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:10:08Z"
concept_id: crates/oxide-library/tests/local_git_adapter/init_refuses_existing_library
language: rust
---

# init_refuses_existing_library

Re-init over an existing `.snxlib` must not silently nuke history.

## Signature

```rust
fn init_refuses_existing_library()
```

## Decorators

- `test`

## Docstring

Re-init over an existing `.snxlib` must not silently nuke history.
[test]

## Source
Lines 102–124 in `crates/oxide-library/tests/local_git_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git_adapter](/crates/oxide-library/tests/local_git_adapter.md) |
| calls | [snxlib_path](/crates/oxide-library/tests/local_git_adapter/snxlib_path.md) |
| calls | [empty_snx_manifest](/crates/oxide-library/tests/local_git_adapter/empty_snx_manifest.md) |
