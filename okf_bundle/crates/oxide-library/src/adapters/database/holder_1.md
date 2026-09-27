---
okf_version: "0.2"
type: Function
title: holder
description: Borrow the holder identity (logged but never the bearer secret).
resource: crates/oxide-library/src/adapters/database.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/database/holder_1
language: rust
---

# holder

Borrow the holder identity (logged but never the bearer secret).

## Signature

```rust
pub fn holder(&self) -> &str
```

## Visibility

- `pub`

## Docstring

Borrow the holder identity (logged but never the bearer secret).

## Source
Lines 153–155 in `crates/oxide-library/src/adapters/database.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [database](/crates/oxide-library/src/adapters/database.md) |
