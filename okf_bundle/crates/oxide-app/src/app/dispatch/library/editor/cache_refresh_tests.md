---
okf_version: "0.2"
type: Module
title: cache_refresh_tests
description: "`refresh_primitive_cache_for` runs immediately after a primitive"
resource: crates/oxide-app/src/app/dispatch/library/editor/cache_refresh_tests.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/editor/cache_refresh_tests
language: rust
---

# cache_refresh_tests

`refresh_primitive_cache_for` runs immediately after a primitive

## Docstring

`refresh_primitive_cache_for` runs immediately after a primitive
save, on caches that are known-good. #599: it overwrote all three
with `unwrap_or_default()`, so a listing failure erased them — the
user saved a symbol and watched every symbol, footprint and sim
vanish from that library's picker while the file sat fine on disk.

## Relationships

| Type | Target |
|------|--------|
| related | [app_with_cached_library](/crates/oxide-app/src/app/dispatch/library/editor/cache_refresh_tests/app_with_cached_library.md) |
| related | [a_post_save_refresh_that_fails_keeps_the_pickers_entries](/crates/oxide-app/src/app/dispatch/library/editor/cache_refresh_tests/a_post_save_refresh_that_fails_keeps_the_pickers_entries.md) |
| related | [a_post_save_refresh_that_fails_reaches_the_messages_panel](/crates/oxide-app/src/app/dispatch/library/editor/cache_refresh_tests/a_post_save_refresh_that_fails_reaches_the_messages_panel.md) |
