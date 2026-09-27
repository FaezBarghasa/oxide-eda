---
okf_version: "0.2"
type: Function
title: health
description: Anonymous liveness endpoints
resource: crates/oxide-library-server/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:36:00Z"
concept_id: crates/oxide-library-server/src/lib/health
language: rust
---

# health

Anonymous liveness endpoints

## Signature

```rust
pub fn health() -> impl Responder
```

## Visibility

- `pub`

## Docstring

Anonymous liveness endpoints

## Source
Lines 44–46 in `crates/oxide-library-server/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-library-server/src/lib.md) |
