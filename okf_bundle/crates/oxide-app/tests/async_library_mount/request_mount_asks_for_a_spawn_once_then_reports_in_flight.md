---
okf_version: "0.2"
type: Function
title: request_mount_asks_for_a_spawn_once_then_reports_in_flight
description: "A cold path with nothing in flight must tell the caller to spawn, and"
resource: crates/oxide-app/tests/async_library_mount.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/async_library_mount/request_mount_asks_for_a_spawn_once_then_reports_in_flight
language: rust
---

# request_mount_asks_for_a_spawn_once_then_reports_in_flight

A cold path with nothing in flight must tell the caller to spawn, and

## Signature

```rust
fn request_mount_asks_for_a_spawn_once_then_reports_in_flight()
```

## Decorators

- `test`

## Docstring

A cold path with nothing in flight must tell the caller to spawn, and
must record the request so a second caller can see it.
[test]

## Source
Lines 121–135 in `crates/oxide-app/tests/async_library_mount.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [async_library_mount](/crates/oxide-app/tests/async_library_mount.md) |
| calls | [fixture](/crates/oxide-app/tests/async_library_mount/fixture.md) |
