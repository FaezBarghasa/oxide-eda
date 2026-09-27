---
okf_version: "0.2"
type: Function
title: take_mount_intent_is_one_shot
description: "`take_mount_intent` consumes. A second completion for the same path"
resource: crates/oxide-app/tests/async_library_mount.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/async_library_mount/take_mount_intent_is_one_shot
language: rust
---

# take_mount_intent_is_one_shot

`take_mount_intent` consumes. A second completion for the same path

## Signature

```rust
fn take_mount_intent_is_one_shot()
```

## Decorators

- `test`

## Docstring

`take_mount_intent` consumes. A second completion for the same path
must not find an intent and act on it twice.
[test]

## Source
Lines 214–226 in `crates/oxide-app/tests/async_library_mount.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [async_library_mount](/crates/oxide-app/tests/async_library_mount.md) |
| calls | [fixture](/crates/oxide-app/tests/async_library_mount/fixture.md) |
