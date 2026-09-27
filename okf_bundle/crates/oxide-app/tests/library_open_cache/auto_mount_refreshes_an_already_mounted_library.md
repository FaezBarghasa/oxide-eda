---
okf_version: "0.2"
type: Function
title: auto_mount_refreshes_an_already_mounted_library
description: "The warm path: a library that is *already* mounted when `auto_mount`"
resource: crates/oxide-app/tests/library_open_cache.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:51:14Z"
concept_id: crates/oxide-app/tests/library_open_cache/auto_mount_refreshes_an_already_mounted_library
language: rust
---

# auto_mount_refreshes_an_already_mounted_library

The warm path: a library that is *already* mounted when `auto_mount`

## Signature

```rust
fn auto_mount_refreshes_an_already_mounted_library()
```

## Decorators

- `test`

## Docstring

The warm path: a library that is *already* mounted when `auto_mount`
reaches it. `open_library` early-returns without reloading anything, so
the `already_open` refresh in `auto_mount_project_libraries` is the only
thing that can notice an out-of-band edit.

Deleting that refresh makes this test fail — which is the whole reason
it is not deleted. It also documents exactly how far the refresh
reaches: primitive directories yes, `.snxlib` rows no.
[test]

## Source
Lines 142–244 in `crates/oxide-app/tests/library_open_cache.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_open_cache](/crates/oxide-app/tests/library_open_cache.md) |
| calls | [generate_library](/crates/oxide-app/tests/support/mod/generate_library.md) |
| calls | [append_component](/crates/oxide-app/tests/support/mod/append_component.md) |
| calls | [project_referencing](/crates/oxide-app/tests/library_open_cache/project_referencing.md) |
| calls | [auto_mount_project_libraries](/crates/oxide-app/src/library/commands/auto_mount_project_libraries.md) |
