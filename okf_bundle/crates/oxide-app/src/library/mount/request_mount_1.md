---
okf_version: "0.2"
type: Function
title: request_mount
description: Record a mount request and report what the caller should do.
resource: crates/oxide-app/src/library/mount.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/mount/request_mount_1
language: rust
---

# request_mount

Record a mount request and report what the caller should do.

## Signature

```rust
pub fn request_mount(&mut self, path: &Path, intent: MountIntent) -> MountRequest
```

## Visibility

- `pub`

## Docstring

Record a mount request and report what the caller should do.

The recorded intent is upgraded in place when a preparation is
already in flight, which is the whole point of the map: the
in-flight preparation finishes and the completion handler reads
the *upgraded* intent rather than one captured at spawn time.

Already-mounted paths record nothing — `library_at` is the same
idempotence guard `open_library` applies at
`state/methods.rs:83-85`.

## Source
Lines 213–227 in `crates/oxide-app/src/library/mount.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mount](/crates/oxide-app/src/library/mount.md) |
