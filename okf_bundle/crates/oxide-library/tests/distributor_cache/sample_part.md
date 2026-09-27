---
okf_version: "0.2"
type: Function
title: sample_part
resource: crates/oxide-library/tests/distributor_cache.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/tests/distributor_cache/sample_part
language: rust
---

# sample_part

## Signature

```rust
fn sample_part(mpn: &str, source: DistributorSource) -> DistributorPart
```

## Source
Lines 13–27 in `crates/oxide-library/tests/distributor_cache.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [distributor_cache](/crates/oxide-library/tests/distributor_cache.md) |
| called_by | [cache_partitions_by_provider](/crates/oxide-library/tests/distributor_cache/cache_partitions_by_provider.md) |
| called_by | [cache_path_layout_matches_spec](/crates/oxide-library/tests/distributor_cache/cache_path_layout_matches_spec.md) |
| called_by | [cache_treats_expired_entry_as_miss](/crates/oxide-library/tests/distributor_cache/cache_treats_expired_entry_as_miss.md) |
| called_by | [cache_write_then_read_returns_same_part](/crates/oxide-library/tests/distributor_cache/cache_write_then_read_returns_same_part.md) |
| called_by | [cache_zero_ttl_always_misses](/crates/oxide-library/tests/distributor_cache/cache_zero_ttl_always_misses.md) |
