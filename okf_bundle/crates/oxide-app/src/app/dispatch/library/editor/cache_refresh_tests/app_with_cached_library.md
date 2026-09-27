---
okf_version: "0.2"
type: Function
title: app_with_cached_library
description: "A mounted library rooted at `<tmp>/<unique>/lib.snxlib` whose three"
resource: crates/oxide-app/src/app/dispatch/library/editor/cache_refresh_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/editor/cache_refresh_tests/app_with_cached_library
language: rust
---

# app_with_cached_library

A mounted library rooted at `<tmp>/<unique>/lib.snxlib` whose three

## Signature

```rust
fn app_with_cached_library() -> (Oxide, PathBuf)
```

## Docstring

A mounted library rooted at `<tmp>/<unique>/lib.snxlib` whose three
primitive caches already hold one entry each, plus the path of a
primitive just saved inside it.

## Source
Lines 18–40 in `crates/oxide-app/src/app/dispatch/library/editor/cache_refresh_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cache_refresh_tests](/crates/oxide-app/src/app/dispatch/library/editor/cache_refresh_tests.md) |
| called_by | [a_post_save_refresh_that_fails_keeps_the_pickers_entries](/crates/oxide-app/src/app/dispatch/library/editor/cache_refresh_tests/a_post_save_refresh_that_fails_keeps_the_pickers_entries.md) |
| called_by | [a_post_save_refresh_that_fails_reaches_the_messages_panel](/crates/oxide-app/src/app/dispatch/library/editor/cache_refresh_tests/a_post_save_refresh_that_fails_reaches_the_messages_panel.md) |
