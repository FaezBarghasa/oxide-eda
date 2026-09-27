---
okf_version: "0.2"
type: Function
title: as_str
description: Borrow the underlying string — for display and for hashing into
resource: crates/oxide-net/src/project/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:01:46Z"
concept_id: crates/oxide-net/src/project/mod/as_str_1
language: rust
---

# as_str

Borrow the underlying string — for display and for hashing into

## Signature

```rust
pub fn as_str(&self) -> &str
```

## Visibility

- `pub`

## Docstring

Borrow the underlying string — for display and for hashing into
caller-side maps, never for path interpretation.

## Source
Lines 76–78 in `crates/oxide-net/src/project/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-net/src/project/mod.md) |
