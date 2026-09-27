---
okf_version: "0.2"
type: Function
title: create_library_at_mounts_with_primed_empty_caches
description: "#530 — the two removals rest on a claim, so the claim gets a test: a"
resource: crates/oxide-app/tests/library_open_cache.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:51:14Z"
concept_id: crates/oxide-app/tests/library_open_cache/create_library_at_mounts_with_primed_empty_caches
language: rust
---

# create_library_at_mounts_with_primed_empty_caches

#530 — the two removals rest on a claim, so the claim gets a test: a

## Signature

```rust
fn create_library_at_mounts_with_primed_empty_caches()
```

## Decorators

- `test`

## Docstring

#530 — the two removals rest on a claim, so the claim gets a test: a
freshly created library is mounted with its caches already primed, and
they are empty because the library is empty.

If `create_library_at` ever stops priming, or starts creating seeded
content, this fails and the dropped `refresh_components` has to come
back.
[test]

## Source
Lines 334–367 in `crates/oxide-app/tests/library_open_cache.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_open_cache](/crates/oxide-app/tests/library_open_cache.md) |
| calls | [project_referencing](/crates/oxide-app/tests/library_open_cache/project_referencing.md) |
| calls | [create_library_at](/crates/oxide-app/src/library/commands/create_library_at.md) |
