---
okf_version: "0.2"
type: Function
title: mount_prepared_does_not_duplicate_an_already_mounted_library
description: "`mount_prepared` carries the same idempotence guard as"
resource: crates/oxide-app/tests/async_library_mount.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/async_library_mount/mount_prepared_does_not_duplicate_an_already_mounted_library
language: rust
---

# mount_prepared_does_not_duplicate_an_already_mounted_library

`mount_prepared` carries the same idempotence guard as

## Signature

```rust
fn mount_prepared_does_not_duplicate_an_already_mounted_library()
```

## Decorators

- `test`

## Docstring

`mount_prepared` carries the same idempotence guard as
`open_library`: a synchronous call site can mount the same path while
a preparation is in flight, and the late arrival must not duplicate
the entry.
[test]

## Source
Lines 233–257 in `crates/oxide-app/tests/async_library_mount.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [async_library_mount](/crates/oxide-app/tests/async_library_mount.md) |
| calls | [fixture](/crates/oxide-app/tests/async_library_mount/fixture.md) |
| calls | [prepare_mount](/crates/oxide-app/src/library/mount/prepare_mount.md) |
