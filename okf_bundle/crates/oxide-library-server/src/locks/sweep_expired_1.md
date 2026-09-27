---
okf_version: "0.2"
type: Function
title: sweep_expired
description: Drop expired entries. Background tasks may call this periodically.
resource: crates/oxide-library-server/src/locks.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library-server/src/locks/sweep_expired_1
language: rust
---

# sweep_expired

Drop expired entries. Background tasks may call this periodically.

## Signature

```rust
pub fn sweep_expired(&self)
```

## Visibility

- `pub`

## Docstring

Drop expired entries. Background tasks may call this periodically.

## Source
Lines 184–193 in `crates/oxide-library-server/src/locks.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [locks](/crates/oxide-library-server/src/locks.md) |
