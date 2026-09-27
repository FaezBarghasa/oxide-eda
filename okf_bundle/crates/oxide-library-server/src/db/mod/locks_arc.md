---
okf_version: "0.2"
type: Function
title: locks_arc
description: "Hand out a clone of the `Arc<LockManager>` for background tasks"
resource: crates/oxide-library-server/src/db/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:44:57Z"
concept_id: crates/oxide-library-server/src/db/mod/locks_arc
language: rust
---

# locks_arc

Hand out a clone of the `Arc<LockManager>` for background tasks

## Signature

```rust
impl AppState { pub fn locks_arc(&self) -> Arc<LockManager> }
```

## Visibility

- `pub`

## Docstring

Hand out a clone of the `Arc<LockManager>` for background tasks
(the periodic `sweep_expired` sweeper holds one of these).

## Source
Lines 106–108 in `crates/oxide-library-server/src/db/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [db](/crates/oxide-library-server/src/db/mod.md) |
