---
okf_version: "0.2"
type: Function
title: snapshot
description: v0.24 Phase 1 — capture the current state into a snapshot
resource: crates/oxide-app/src/app/documents.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/documents/snapshot_1
language: rust
---

# snapshot

v0.24 Phase 1 — capture the current state into a snapshot

## Signature

```rust
fn snapshot(&self) -> FootprintHistorySnapshot
```

## Docstring

v0.24 Phase 1 — capture the current state into a snapshot
without altering the history stack. Used by `push_history`
+ `undo`/`redo` to read the canonical state by-value.

## Source
Lines 543–552 in `crates/oxide-app/src/app/documents.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [documents](/crates/oxide-app/src/app/documents.md) |
