---
okf_version: "0.2"
type: Function
title: handle_close_library
description: Close an open library — diverts to the confirm modal when any
resource: crates/oxide-app/src/app/dispatch/library/lifecycle.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/lifecycle/handle_close_library
language: rust
---

# handle_close_library

Close an open library — diverts to the confirm modal when any

## Signature

```rust
impl Oxide { pub(super) fn handle_close_library(&mut self, path: std::path::PathBuf) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Docstring

Close an open library — diverts to the confirm modal when any
Component Preview editor against it is dirty.

## Source
Lines 114–141 in `crates/oxide-app/src/app/dispatch/library/lifecycle.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lifecycle](/crates/oxide-app/src/app/dispatch/library/lifecycle.md) |
