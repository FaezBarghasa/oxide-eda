---
okf_version: "0.2"
type: Module
title: distributor_cache
description: "Integration tests for `DistributorCache`. Cache round-trip tests"
resource: crates/oxide-library/tests/distributor_cache.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/tests/distributor_cache
language: rust
---

# distributor_cache

Integration tests for `DistributorCache`. Cache round-trip tests

## Docstring

Integration tests for `DistributorCache`. Cache round-trip tests
must run **without network**.

## Relationships

| Type | Target |
|------|--------|
| related | [sample_part](/crates/oxide-library/tests/distributor_cache/sample_part.md) |
| related | [cache_write_then_read_returns_same_part](/crates/oxide-library/tests/distributor_cache/cache_write_then_read_returns_same_part.md) |
| related | [cache_miss_when_entry_absent](/crates/oxide-library/tests/distributor_cache/cache_miss_when_entry_absent.md) |
| related | [cache_treats_expired_entry_as_miss](/crates/oxide-library/tests/distributor_cache/cache_treats_expired_entry_as_miss.md) |
| related | [cache_zero_ttl_always_misses](/crates/oxide-library/tests/distributor_cache/cache_zero_ttl_always_misses.md) |
| related | [cache_partitions_by_provider](/crates/oxide-library/tests/distributor_cache/cache_partitions_by_provider.md) |
| related | [cache_path_layout_matches_spec](/crates/oxide-library/tests/distributor_cache/cache_path_layout_matches_spec.md) |
| related | [chrono](/_dependencies/cargo/chrono.md) |
