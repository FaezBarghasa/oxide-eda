---
okf_version: "0.2"
type: Function
title: prepare_mount_then_mount_prepared_matches_open_library
description: "The whole point of the change: preparing off-thread and mounting the"
resource: crates/oxide-app/tests/async_library_mount.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/async_library_mount/prepare_mount_then_mount_prepared_matches_open_library
language: rust
---

# prepare_mount_then_mount_prepared_matches_open_library

The whole point of the change: preparing off-thread and mounting the

## Signature

```rust
fn prepare_mount_then_mount_prepared_matches_open_library()
```

## Decorators

- `test`

## Docstring

The whole point of the change: preparing off-thread and mounting the
result must leave `LibraryState` in the state the synchronous
`open_library` left it in.

Compared against the synchronous path rather than against literal
counts, so it still means something after the cache shape changes.
[test]

## Source
Lines 55–116 in `crates/oxide-app/tests/async_library_mount.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [async_library_mount](/crates/oxide-app/tests/async_library_mount.md) |
| calls | [fixture](/crates/oxide-app/tests/async_library_mount/fixture.md) |
| calls | [prepare_mount](/crates/oxide-app/src/library/mount/prepare_mount.md) |
