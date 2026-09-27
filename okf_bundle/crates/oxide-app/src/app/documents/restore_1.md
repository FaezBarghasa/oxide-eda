---
okf_version: "0.2"
type: Function
title: restore
description: v0.24 Phase 1 — write a snapshot back into the live editor
resource: crates/oxide-app/src/app/documents.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/documents/restore_1
language: rust
---

# restore

v0.24 Phase 1 — write a snapshot back into the live editor

## Signature

```rust
fn restore(&mut self, snap: FootprintHistorySnapshot)
```

## Docstring

v0.24 Phase 1 — write a snapshot back into the live editor
state. Resets the canvas cache so the next render reflects
the rolled-back geometry.

## Source
Lines 557–566 in `crates/oxide-app/src/app/documents.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [documents](/crates/oxide-app/src/app/documents.md) |
