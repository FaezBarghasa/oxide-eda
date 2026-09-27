---
okf_version: "0.2"
type: Function
title: main
description: "[actix_web::main]"
resource: crates/oxide-library-server/src/main.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:36:34Z"
concept_id: crates/oxide-library-server/src/main/main
language: rust
---

# main

[actix_web::main]

## Signature

```rust
fn main() -> anyhow::Result<()>
```

## Decorators

- `actix_web::main`

## Docstring

[actix_web::main]

## Source
Lines 21–97 in `crates/oxide-library-server/src/main.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [src](/crates/oxide-library-server/src/main.md) |
| calls | [is_loopback_bind](/crates/oxide-library-server/src/main/is_loopback_bind.md) |
| calls | [start_lock_sweeper](/crates/oxide-library-server/src/lib/start_lock_sweeper.md) |
| calls | [default_cors](/crates/oxide-library-server/src/lib/default_cors.md) |
