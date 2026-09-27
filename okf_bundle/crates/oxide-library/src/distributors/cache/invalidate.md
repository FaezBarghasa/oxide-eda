---
okf_version: "0.2"
type: Function
title: invalidate
description: "Delete a cached entry, if present. Idempotent."
resource: crates/oxide-library/src/distributors/cache.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/distributors/cache/invalidate
language: rust
---

# invalidate

Delete a cached entry, if present. Idempotent.

## Signature

```rust
impl DistributorCache { pub fn invalidate(&self, provider: &str, mpn: &str) -> Result<(), CacheError> }
```

## Visibility

- `pub`

## Docstring

Delete a cached entry, if present. Idempotent.

## Source
Lines 141–148 in `crates/oxide-library/src/distributors/cache.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cache](/crates/oxide-library/src/distributors/cache.md) |
