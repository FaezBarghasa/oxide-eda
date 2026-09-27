---
okf_version: "0.2"
type: Function
title: request_mount_reports_already_mounted_and_records_nothing
description: An already-mounted library is neither a spawn nor an in-flight wait —
resource: crates/oxide-app/tests/async_library_mount.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/async_library_mount/request_mount_reports_already_mounted_and_records_nothing
language: rust
---

# request_mount_reports_already_mounted_and_records_nothing

An already-mounted library is neither a spawn nor an in-flight wait —

## Signature

```rust
fn request_mount_reports_already_mounted_and_records_nothing()
```

## Decorators

- `test`

## Docstring

An already-mounted library is neither a spawn nor an in-flight wait —
the caller has to act immediately. Collapsing this into either of the
other two outcomes loses the tab or opens it twice.
[test]

## Source
Lines 176–191 in `crates/oxide-app/tests/async_library_mount.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [async_library_mount](/crates/oxide-app/tests/async_library_mount.md) |
| calls | [fixture](/crates/oxide-app/tests/async_library_mount/fixture.md) |
