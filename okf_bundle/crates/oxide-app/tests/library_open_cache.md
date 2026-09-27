---
okf_version: "0.2"
type: Module
title: library_open_cache
description: "Both halves of the #99 part-2a cache contract for"
resource: crates/oxide-app/tests/library_open_cache.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:51:14Z"
concept_id: crates/oxide-app/tests/library_open_cache
language: rust
---

# library_open_cache

Both halves of the #99 part-2a cache contract for

## Docstring

Both halves of the #99 part-2a cache contract for
`auto_mount_project_libraries`.

**Cold path.** `auto_mount` no longer chases each `open_library` with a
`refresh_components` that recomputed the identical five fields off the
identical adapter calls — that duplicate roughly doubled every project
open. `open_library` is now the sole thing standing between a mounted
library and a populated panel, so
`open_library_primes_every_cache` pins that it fills all five, and
`refresh_components_after_open_changes_nothing` pins that the dropped
call really was a no-op there.

**Warm path.** `open_library` early-returns at
`library/state/methods.rs:83-85` when the library is already mounted and
never reaches `reload_tables`, so for an already-open library the
refresh is the *only* thing that rescans anything.
`auto_mount_refreshes_an_already_mounted_library` pins that — it is the
case a blanket deletion would have silently broken.

That test also pins the limit of what the refresh can do: it rescans the
primitive *directories*, but cannot see rows added to the `.snxlib`
itself, because the mounted `LocalGitAdapter` serves tables from an
in-memory parse taken at `open()`. See the comments on its assertions.

Between them: trimming `reload_tables` / `reload_primitives` out of the
open path fails here, and so does deleting the warm-path refresh, rather
than the UI silently rendering a stale or empty library.

## Relationships

| Type | Target |
|------|--------|
| related | [open_library_primes_every_cache](/crates/oxide-app/tests/library_open_cache/open_library_primes_every_cache.md) |
| related | [refresh_components_after_open_changes_nothing](/crates/oxide-app/tests/library_open_cache/refresh_components_after_open_changes_nothing.md) |
| related | [auto_mount_refreshes_an_already_mounted_library](/crates/oxide-app/tests/library_open_cache/auto_mount_refreshes_an_already_mounted_library.md) |
| related | [project_referencing](/crates/oxide-app/tests/library_open_cache/project_referencing.md) |
| related | [commands_open_library_refreshes_an_already_mounted_library](/crates/oxide-app/tests/library_open_cache/commands_open_library_refreshes_an_already_mounted_library.md) |
| related | [create_library_at_mounts_with_primed_empty_caches](/crates/oxide-app/tests/library_open_cache/create_library_at_mounts_with_primed_empty_caches.md) |
| related | [snapshot](/crates/oxide-app/tests/library_open_cache/snapshot.md) |
