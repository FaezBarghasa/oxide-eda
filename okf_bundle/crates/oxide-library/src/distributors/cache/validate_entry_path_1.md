---
okf_version: "0.2"
type: Function
title: validate_entry_path
description: "M2: enforce that `mpn` produces a path strictly inside `self.root`."
resource: crates/oxide-library/src/distributors/cache.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/distributors/cache/validate_entry_path_1
language: rust
---

# validate_entry_path

M2: enforce that `mpn` produces a path strictly inside `self.root`.

## Signature

```rust
fn validate_entry_path(&self, provider: &str, mpn: &str) -> Result<PathBuf, CacheError>
```

## Docstring

M2: enforce that `mpn` produces a path strictly inside `self.root`.
Any rejection bubbles up as `CacheError::Io(InvalidInput)` so callers
can branch on it without parsing strings.

## Source
Lines 81–101 in `crates/oxide-library/src/distributors/cache.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cache](/crates/oxide-library/src/distributors/cache.md) |
