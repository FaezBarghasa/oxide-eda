---
okf_version: "0.2"
type: Function
title: set_idle_ttl
description: "Override TTL — primarily for tests so they don't have to wait minutes."
resource: crates/oxide-library-server/src/locks.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library-server/src/locks/set_idle_ttl_1
language: rust
---

# set_idle_ttl

Override TTL — primarily for tests so they don't have to wait minutes.

## Signature

```rust
pub fn set_idle_ttl(&self, ttl: Duration)
```

## Visibility

- `pub`

## Docstring

Override TTL — primarily for tests so they don't have to wait minutes.

## Source
Lines 90–96 in `crates/oxide-library-server/src/locks.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [locks](/crates/oxide-library-server/src/locks.md) |
