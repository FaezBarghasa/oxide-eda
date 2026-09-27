---
okf_version: "0.2"
type: Module
title: test_adapters
description: "Test-only [`LibraryAdapter`] stubs."
resource: crates/oxide-app/src/library/test_adapters.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/test_adapters
language: rust
---

# test_adapters

Test-only [`LibraryAdapter`] stubs.

## Docstring

Test-only [`LibraryAdapter`] stubs.

Shared by the cache-refresh regressions in `library::state` and in
`app::dispatch::library::editor`, which both need a mounted library
whose primitive listings fail.

## Relationships

| Type | Target |
|------|--------|
| related | [FailingListingAdapter](/crates/oxide-app/src/library/test_adapters/FailingListingAdapter.md) |
| related | [new](/crates/oxide-app/src/library/test_adapters/new.md) |
| related | [listing_failure](/crates/oxide-app/src/library/test_adapters/listing_failure.md) |
| related | [new](/crates/oxide-app/src/library/test_adapters/new.md) |
| related | [listing_failure](/crates/oxide-app/src/library/test_adapters/listing_failure.md) |
| related | [manifest](/crates/oxide-app/src/library/test_adapters/manifest.md) |
| related | [list_tables](/crates/oxide-app/src/library/test_adapters/list_tables.md) |
| related | [list_symbols](/crates/oxide-app/src/library/test_adapters/list_symbols.md) |
| related | [list_footprints](/crates/oxide-app/src/library/test_adapters/list_footprints.md) |
| related | [list_sims](/crates/oxide-app/src/library/test_adapters/list_sims.md) |
| related | [manifest](/crates/oxide-app/src/library/test_adapters/manifest.md) |
| related | [list_tables](/crates/oxide-app/src/library/test_adapters/list_tables.md) |
| related | [list_symbols](/crates/oxide-app/src/library/test_adapters/list_symbols.md) |
| related | [list_footprints](/crates/oxide-app/src/library/test_adapters/list_footprints.md) |
| related | [list_sims](/crates/oxide-app/src/library/test_adapters/list_sims.md) |
| related | [cached_summary](/crates/oxide-app/src/library/test_adapters/cached_summary.md) |
