---
okf_version: "0.2"
type: Function
title: take
description: "Take the payload. `None` on a second read — see the type docs."
resource: crates/oxide-app/src/library/mount.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/mount/take_1
language: rust
---

# take

Take the payload. `None` on a second read — see the type docs.

## Signature

```rust
pub fn take(&self) -> Option<Result<PreparedMount, String>>
```

## Visibility

- `pub`

## Docstring

Take the payload. `None` on a second read — see the type docs.

A poisoned lock is recovered rather than propagated: the only
writer is this type's own `take`, so poisoning means a panic
happened elsewhere while the guard was held and the `Option` is
still structurally sound.

## Source
Lines 114–119 in `crates/oxide-app/src/library/mount.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mount](/crates/oxide-app/src/library/mount.md) |
