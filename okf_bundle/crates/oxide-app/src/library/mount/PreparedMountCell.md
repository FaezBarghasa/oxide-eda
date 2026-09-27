---
okf_version: "0.2"
type: Class
title: PreparedMountCell
description: "One-shot carrier for a [`PreparedMount`] across the `Task::perform`"
resource: crates/oxide-app/src/library/mount.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/mount/PreparedMountCell
language: rust
---

# PreparedMountCell

One-shot carrier for a [`PreparedMount`] across the `Task::perform`

## Signature

```rust
pub struct PreparedMountCell
```

## Decorators

- `derive(Clone)`

## Visibility

- `pub`

## Docstring

One-shot carrier for a [`PreparedMount`] across the `Task::perform`
boundary.

`LibraryMessage` derives `Debug + Clone`; `PreparedMount` has
neither, because `Box<dyn LibraryAdapter>` has neither. The `Arc`
makes the *message* cloneable without making the payload cloneable,
and [`take`](Self::take) enforces the one-shot contract: whichever
clone is dispatched first gets the payload and every later clone sees
`None`. iced does not clone a `Task::perform` result today, so the
second read should never happen — it degrades to `None` rather than a
panic because a dropped mount is a stale cache, not a corrupt one.
[derive(Clone)]

## Source
Lines 101–101 in `crates/oxide-app/src/library/mount.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mount](/crates/oxide-app/src/library/mount.md) |
