---
okf_version: "0.2"
type: Function
title: start_lock_sweeper
description: Spawns the background task to clean expired advisory locks.
resource: crates/oxide-library-server/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:36:00Z"
concept_id: crates/oxide-library-server/src/lib/start_lock_sweeper
language: rust
---

# start_lock_sweeper

Spawns the background task to clean expired advisory locks.

## Signature

```rust
pub fn start_lock_sweeper(state: &AppState)
```

## Visibility

- `pub`

## Docstring

Spawns the background task to clean expired advisory locks.

## Source
Lines 70–80 in `crates/oxide-library-server/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-library-server/src/lib.md) |
| called_by | [main](/crates/oxide-library-server/src/main/main.md) |
