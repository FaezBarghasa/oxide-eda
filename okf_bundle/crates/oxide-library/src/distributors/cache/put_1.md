---
okf_version: "0.2"
type: Function
title: put
description: "Write a part to the cache. Refreshes `captured_at` is the caller's"
resource: crates/oxide-library/src/distributors/cache.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/distributors/cache/put_1
language: rust
---

# put

Write a part to the cache. Refreshes `captured_at` is the caller's

## Signature

```rust
pub fn put(&self, provider: &str, part: &DistributorPart) -> Result<(), CacheError>
```

## Visibility

- `pub`

## Docstring

Write a part to the cache. Refreshes `captured_at` is the caller's
responsibility — we persist whatever is on the part.

## Source
Lines 105–113 in `crates/oxide-library/src/distributors/cache.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cache](/crates/oxide-library/src/distributors/cache.md) |
