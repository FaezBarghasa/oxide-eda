---
okf_version: "0.2"
type: Function
title: mount_prepared
description: "Finish a mount prepared off-thread. Cheap: a `LibrarySet::mount`"
resource: crates/oxide-app/src/library/mount.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/mount/mount_prepared_1
language: rust
---

# mount_prepared

Finish a mount prepared off-thread. Cheap: a `LibrarySet::mount`

## Signature

```rust
pub fn mount_prepared(&mut self, prepared: PreparedMount) -> Result<(), LibraryError>
```

## Visibility

- `pub`

## Docstring

Finish a mount prepared off-thread. Cheap: a `LibrarySet::mount`
plus two pushes, no disk IO.

Idempotent in the same way [`LibraryState::open_library`] is — a
path already present wins and the prepared adapter is dropped.
That is reachable: a synchronous `open_library` call site can
mount the same path while this preparation was in flight.

## Source
Lines 250–264 in `crates/oxide-app/src/library/mount.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mount](/crates/oxide-app/src/library/mount.md) |
