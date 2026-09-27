---
okf_version: "0.2"
type: Module
title: cache
description: "Disk JSON cache for `DistributorPart` records."
resource: crates/oxide-library/src/distributors/cache.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/distributors/cache
language: rust
---

# cache

Disk JSON cache for `DistributorPart` records.

## Docstring

Disk JSON cache for `DistributorPart` records.

- Layout: `<root>/<provider>/<mpn>.json` (one JSON file per part).
- Default TTL: **24 hours** for metadata. Datasheet URLs cached
indefinitely (the URL is stored as part of the same JSON; the
indefinite-ness is a property of the URL not changing — we re-read
stale entries explicitly when the caller wants a refresh).
- Tests use a temp dir, never `~/.oxide/`.

`DistributorCache::with_root` is the test-friendly constructor;
`DistributorCache::default_root()` resolves `~/.oxide/cache/distributor`
for runtime use.

## Relationships

| Type | Target |
|------|--------|
| related | [CacheError](/crates/oxide-library/src/distributors/cache/CacheError.md) |
| related | [DistributorCache](/crates/oxide-library/src/distributors/cache/DistributorCache.md) |
| related | [with_root](/crates/oxide-library/src/distributors/cache/with_root.md) |
| related | [default_root](/crates/oxide-library/src/distributors/cache/default_root.md) |
| related | [entry_path](/crates/oxide-library/src/distributors/cache/entry_path.md) |
| related | [validate_entry_path](/crates/oxide-library/src/distributors/cache/validate_entry_path.md) |
| related | [put](/crates/oxide-library/src/distributors/cache/put.md) |
| related | [get](/crates/oxide-library/src/distributors/cache/get.md) |
| related | [invalidate](/crates/oxide-library/src/distributors/cache/invalidate.md) |
| related | [root](/crates/oxide-library/src/distributors/cache/root.md) |
| related | [with_root](/crates/oxide-library/src/distributors/cache/with_root.md) |
| related | [default_root](/crates/oxide-library/src/distributors/cache/default_root.md) |
| related | [entry_path](/crates/oxide-library/src/distributors/cache/entry_path.md) |
| related | [validate_entry_path](/crates/oxide-library/src/distributors/cache/validate_entry_path.md) |
| related | [put](/crates/oxide-library/src/distributors/cache/put.md) |
| related | [get](/crates/oxide-library/src/distributors/cache/get.md) |
| related | [invalidate](/crates/oxide-library/src/distributors/cache/invalidate.md) |
| related | [root](/crates/oxide-library/src/distributors/cache/root.md) |
| related | [part](/crates/oxide-library/src/distributors/cache/part.md) |
| related | [entry_path_sanitises_slashes](/crates/oxide-library/src/distributors/cache/entry_path_sanitises_slashes.md) |
| related | [invalidate_is_idempotent](/crates/oxide-library/src/distributors/cache/invalidate_is_idempotent.md) |
| related | [entry_path_rejects_parent_dir_traversal](/crates/oxide-library/src/distributors/cache/entry_path_rejects_parent_dir_traversal.md) |
| related | [chrono](/_dependencies/cargo/chrono.md) |
