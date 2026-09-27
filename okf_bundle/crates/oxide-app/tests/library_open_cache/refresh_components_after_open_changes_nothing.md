---
okf_version: "0.2"
type: Function
title: refresh_components_after_open_changes_nothing
description: "The stronger claim behind the deletion: a `refresh_components` run"
resource: crates/oxide-app/tests/library_open_cache.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:51:14Z"
concept_id: crates/oxide-app/tests/library_open_cache/refresh_components_after_open_changes_nothing
language: rust
---

# refresh_components_after_open_changes_nothing

The stronger claim behind the deletion: a `refresh_components` run

## Signature

```rust
fn refresh_components_after_open_changes_nothing()
```

## Decorators

- `test`

## Docstring

The stronger claim behind the deletion: a `refresh_components` run
straight after `open_library` is a no-op on the cache contents. If this
ever stops holding, the dropped call in `auto_mount_project_libraries`
was load-bearing after all.
[test]

## Source
Lines 110–131 in `crates/oxide-app/tests/library_open_cache.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_open_cache](/crates/oxide-app/tests/library_open_cache.md) |
| calls | [generate_library](/crates/oxide-app/tests/support/mod/generate_library.md) |
| calls | [snapshot](/crates/oxide-app/tests/library_open_cache/snapshot.md) |
