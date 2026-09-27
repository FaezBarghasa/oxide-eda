---
okf_version: "0.2"
type: Class
title: CanonView
description: Canonical serialisation view — only the fields the hash should care about.
resource: crates/oxide-library/src/hash.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/hash/CanonView
language: rust
---

# CanonView

Canonical serialisation view — only the fields the hash should care about.

## Signature

```rust
struct CanonView
```

## Type Parameters

- `'a`

## Decorators

- `derive(Serialize)`

## Docstring

Canonical serialisation view — only the fields the hash should care about.

`row_id` IS hashed: a row's identity is part of its content. `internal_pn`
and `class` are user-renameable fields and so they ARE hashed (changing
either is a real content change). Bookkeeping (`created` / `updated` /
`content_hash`) is omitted.
[derive(Serialize)]

## Methods

- `row_id`
- `internal_pn`
- `class`
- `state`
- `datasheet`
- `symbol_ref`
- `footprint_ref`
- `sim_ref`
- `pin_map_overrides`
- `primary_mpn`
- `alternates`
- `supply`
- `parameters`
- `plm`

## Source
Lines 36–51 in `crates/oxide-library/src/hash.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hash](/crates/oxide-library/src/hash.md) |
