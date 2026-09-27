---
okf_version: "0.2"
type: Function
title: deny
description: Windows directories ignore the FILE_ATTRIBUTE_READONLY bit for new-file
resource: crates/oxide-app/src/test_support.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/test_support/deny_1
language: rust
---

# deny

Windows directories ignore the FILE_ATTRIBUTE_READONLY bit for new-file

## Signature

```rust
fn deny(dir: &Path)
```

## Decorators

- `cfg(windows)`

## Docstring

Windows directories ignore the FILE_ATTRIBUTE_READONLY bit for new-file
creation inside them, so `Permissions::set_readonly` can't simulate this —
deny the current user's "create files" ACE instead.
[cfg(windows)]

## Source
Lines 54–64 in `crates/oxide-app/src/test_support.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [test_support](/crates/oxide-app/src/test_support.md) |
| calls | [settle_deny](/crates/oxide-app/src/test_support/settle_deny.md) |
