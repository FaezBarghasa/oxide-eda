---
okf_version: "0.2"
type: Function
title: a_double_click_during_a_silent_mount_upgrades_the_intent
description: "The subtle half of the design. A project auto-mount records `Silent`;"
resource: crates/oxide-app/tests/async_library_mount.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/async_library_mount/a_double_click_during_a_silent_mount_upgrades_the_intent
language: rust
---

# a_double_click_during_a_silent_mount_upgrades_the_intent

The subtle half of the design. A project auto-mount records `Silent`;

## Signature

```rust
fn a_double_click_during_a_silent_mount_upgrades_the_intent()
```

## Decorators

- `test`

## Docstring

The subtle half of the design. A project auto-mount records `Silent`;
the user then double-clicks the same `.snxlib` while that preparation
is still running. The tab must still open, so the intent escalates.
[test]

## Source
Lines 141–153 in `crates/oxide-app/tests/async_library_mount.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [async_library_mount](/crates/oxide-app/tests/async_library_mount.md) |
| calls | [fixture](/crates/oxide-app/tests/async_library_mount/fixture.md) |
