---
okf_version: "0.2"
type: Class
title: MountRequest
description: "What [`LibraryState::request_mount`] decided."
resource: crates/oxide-app/src/library/mount.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/mount/MountRequest
language: rust
---

# MountRequest

What [`LibraryState::request_mount`] decided.

## Signature

```rust
pub enum MountRequest
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

What [`LibraryState::request_mount`] decided.

Three outcomes, not two: "already mounted" and "a preparation is in
flight" both mean *do not spawn*, but they need opposite follow-ups.
An already-mounted path must act **now** (the library is there, so a
browser tab can open immediately), while an in-flight path must
**not** act now — the completion handler will, once the preparation
lands. Collapsing them into one `false` either loses the tab or opens
it twice.
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Source
Lines 190–200 in `crates/oxide-app/src/library/mount.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mount](/crates/oxide-app/src/library/mount.md) |
