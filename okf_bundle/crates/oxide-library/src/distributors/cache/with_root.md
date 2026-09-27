---
okf_version: "0.2"
type: Function
title: with_root
description: Construct a cache at the given root directory. Creates the root if it
resource: crates/oxide-library/src/distributors/cache.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/distributors/cache/with_root
language: rust
---

# with_root

Construct a cache at the given root directory. Creates the root if it

## Signature

```rust
impl DistributorCache { pub fn with_root(root: impl AsRef<Path>) -> Result<Self, CacheError> }
```

## Visibility

- `pub`

## Docstring

Construct a cache at the given root directory. Creates the root if it
does not exist. Test-friendly: pass a `tempfile::TempDir` path.

## Source
Lines 43–47 in `crates/oxide-library/src/distributors/cache.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cache](/crates/oxide-library/src/distributors/cache.md) |
