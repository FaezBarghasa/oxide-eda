---
okf_version: "0.2"
type: Function
title: default_root
description: "Resolve `~/.oxide/cache/distributor` (creates it on first use)."
resource: crates/oxide-library/src/distributors/cache.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/distributors/cache/default_root_1
language: rust
---

# default_root

Resolve `~/.oxide/cache/distributor` (creates it on first use).

## Signature

```rust
pub fn default_root() -> Result<Self, CacheError>
```

## Visibility

- `pub`

## Docstring

Resolve `~/.oxide/cache/distributor` (creates it on first use).
Production callers use this; tests should call [`Self::with_root`].

## Source
Lines 51–55 in `crates/oxide-library/src/distributors/cache.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cache](/crates/oxide-library/src/distributors/cache.md) |
