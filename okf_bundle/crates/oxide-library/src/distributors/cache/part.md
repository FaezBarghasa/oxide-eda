---
okf_version: "0.2"
type: Function
title: part
resource: crates/oxide-library/src/distributors/cache.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/distributors/cache/part
language: rust
---

# part

## Signature

```rust
fn part(mpn: &str) -> DistributorPart
```

## Source
Lines 162–176 in `crates/oxide-library/src/distributors/cache.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cache](/crates/oxide-library/src/distributors/cache.md) |
| called_by | [entry_path_rejects_parent_dir_traversal](/crates/oxide-library/src/distributors/cache/entry_path_rejects_parent_dir_traversal.md) |
| called_by | [invalidate_is_idempotent](/crates/oxide-library/src/distributors/cache/invalidate_is_idempotent.md) |
