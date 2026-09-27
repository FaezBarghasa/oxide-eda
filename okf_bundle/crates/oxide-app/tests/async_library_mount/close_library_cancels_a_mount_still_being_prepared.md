---
okf_version: "0.2"
type: Function
title: close_library_cancels_a_mount_still_being_prepared
description: Closing a library while its preparation is in flight has to cancel it.
resource: crates/oxide-app/tests/async_library_mount.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/async_library_mount/close_library_cancels_a_mount_still_being_prepared
language: rust
---

# close_library_cancels_a_mount_still_being_prepared

Closing a library while its preparation is in flight has to cancel it.

## Signature

```rust
fn close_library_cancels_a_mount_still_being_prepared()
```

## Decorators

- `test`

## Docstring

Closing a library while its preparation is in flight has to cancel it.
Without the tombstone the completion would re-mount a library the user
just closed, and it would reappear with no gesture behind it.
[test]

## Source
Lines 197–209 in `crates/oxide-app/tests/async_library_mount.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [async_library_mount](/crates/oxide-app/tests/async_library_mount.md) |
| calls | [fixture](/crates/oxide-app/tests/async_library_mount/fixture.md) |
