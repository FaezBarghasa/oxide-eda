---
okf_version: "0.2"
type: Function
title: get
description: "Read a cached part if it exists and is fresher than `ttl`."
resource: crates/oxide-library/src/distributors/cache.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/distributors/cache/get_1
language: rust
---

# get

Read a cached part if it exists and is fresher than `ttl`.

## Signature

```rust
pub fn get(
        &self,
        provider: &str,
        mpn: &str,
        ttl: Duration,
    ) -> Result<Option<DistributorPart>, CacheError>
```

## Visibility

- `pub`

## Docstring

Read a cached part if it exists and is fresher than `ttl`.
Returns `Ok(None)` for cache miss or stale entry.

## Source
Lines 117–138 in `crates/oxide-library/src/distributors/cache.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cache](/crates/oxide-library/src/distributors/cache.md) |
