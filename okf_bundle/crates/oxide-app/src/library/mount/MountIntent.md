---
okf_version: "0.2"
type: Class
title: MountIntent
description: Why a mount was requested — decides what the completion handler does
resource: crates/oxide-app/src/library/mount.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/mount/MountIntent
language: rust
---

# MountIntent

Why a mount was requested — decides what the completion handler does

## Signature

```rust
pub enum MountIntent
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Why a mount was requested — decides what the completion handler does
once the library is on [`LibraryState`].

A mount already in flight for a path can be re-requested with a
*different* intent, and the stronger one has to win. Double-clicking
a `.snxlib` while a project auto-mount for the same path is still
preparing must still open the browser tab, so `Silent` upgrades to
`OpenBrowserTab` — never the reverse.
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Source
Lines 48–55 in `crates/oxide-app/src/library/mount.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mount](/crates/oxide-app/src/library/mount.md) |
