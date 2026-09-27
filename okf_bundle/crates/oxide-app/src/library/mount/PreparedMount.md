---
okf_version: "0.2"
type: Class
title: PreparedMount
description: "Everything the UI thread needs to finish a mount, built entirely off"
resource: crates/oxide-app/src/library/mount.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/mount/PreparedMount
language: rust
---

# PreparedMount

Everything the UI thread needs to finish a mount, built entirely off

## Signature

```rust
pub struct PreparedMount
```

## Visibility

- `pub`

## Docstring

Everything the UI thread needs to finish a mount, built entirely off
the UI thread.

`entry`'s five caches are already primed here — `reload_tables`
populates `tables` + `cached_components` and then chains
`reload_primitives` for the three primitive listings. That is the
same priming the synchronous [`LibraryState::open_library`] does, so
the cold-path invariant #528 pinned still holds: nothing downstream
needs a chaser refresh.

## Methods

- `adapter`
- `entry`

## Source
Lines 77–80 in `crates/oxide-app/src/library/mount.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mount](/crates/oxide-app/src/library/mount.md) |
