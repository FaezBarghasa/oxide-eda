---
okf_version: "0.2"
type: Function
title: library_with_cached_primitives
description: "An `OpenLibrary` whose three primitive caches already hold one"
resource: crates/oxide-app/src/library/state/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/state/tests/library_with_cached_primitives
language: rust
---

# library_with_cached_primitives

An `OpenLibrary` whose three primitive caches already hold one

## Signature

```rust
fn library_with_cached_primitives(root: &Path, library_id: Uuid) -> OpenLibrary
```

## Docstring

An `OpenLibrary` whose three primitive caches already hold one
entry each — the state a mounted library is in before any refresh.

## Source
Lines 228–240 in `crates/oxide-app/src/library/state/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/state/tests.md) |
| called_by | [a_failed_listing_keeps_the_primitive_caches_it_could_not_refresh](/crates/oxide-app/src/library/state/tests/a_failed_listing_keeps_the_primitive_caches_it_could_not_refresh.md) |
| called_by | [a_failed_listing_reaches_the_messages_panel](/crates/oxide-app/src/library/state/tests/a_failed_listing_reaches_the_messages_panel.md) |
| called_by | [refresh_components_keeps_the_caches_when_the_listings_fail](/crates/oxide-app/src/library/state/tests/refresh_components_keeps_the_caches_when_the_listings_fail.md) |
