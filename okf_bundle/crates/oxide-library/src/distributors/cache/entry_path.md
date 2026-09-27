---
okf_version: "0.2"
type: Function
title: entry_path
description: "Compute the on-disk path for a `(provider, mpn)` entry."
resource: crates/oxide-library/src/distributors/cache.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/distributors/cache/entry_path
language: rust
---

# entry_path

Compute the on-disk path for a `(provider, mpn)` entry.

## Signature

```rust
impl DistributorCache { pub fn entry_path(&self, provider: &str, mpn: &str) -> PathBuf }
```

## Visibility

- `pub`

## Docstring

Compute the on-disk path for a `(provider, mpn)` entry.

M2: directory-traversal hardening. The MPN is partially trusted
(vendor-supplied via API responses we don't control), so we:
1. Replace path separators with `_` (preserves the original `/`/`\`
sanitisation that allowed legitimate slashes in MPNs).
2. Reject any MPN containing `..` (parent-dir escape).
3. Verify the canonicalised result still lives under `self.root`.

On rejection the path can't be derived; callers should treat the
error as "this MPN cannot be cached" and return live results
without persisting them.

## Source
Lines 69–76 in `crates/oxide-library/src/distributors/cache.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cache](/crates/oxide-library/src/distributors/cache.md) |
