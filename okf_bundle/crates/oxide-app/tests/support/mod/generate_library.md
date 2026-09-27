---
okf_version: "0.2"
type: Function
title: generate_library
description: "Build a full `.snxlib` under `<root>/<name>/`, returning the path to the"
resource: crates/oxide-app/tests/support/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/support/mod/generate_library
language: rust
---

# generate_library

Build a full `.snxlib` under `<root>/<name>/`, returning the path to the

## Signature

```rust
pub fn generate_library(root: &Path, name: &str, scale: &Scale) -> Result<PathBuf, LibraryError>
```

## Visibility

- `pub`

## Docstring

Build a full `.snxlib` under `<root>/<name>/`, returning the path to the
`.snxlib` file itself. Verifies the result opens and lists correctly
through `LocalGitAdapter` before returning — a subtly invalid library
would silently invalidate every number downstream.

## Source
Lines 348–429 in `crates/oxide-app/tests/support/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [support](/crates/oxide-app/tests/support/mod.md) |
| calls | [manifest](/crates/oxide-app/tests/support/mod/manifest.md) |
| calls | [library_id](/crates/oxide-library/src/adapter/library_id.md) |
| calls | [make_symbol](/crates/oxide-app/tests/support/mod/make_symbol.md) |
| calls | [make_footprint](/crates/oxide-app/tests/support/mod/make_footprint.md) |
| calls | [make_sim](/crates/oxide-app/tests/support/mod/make_sim.md) |
| calls | [make_row](/crates/oxide-app/tests/support/mod/make_row.md) |
| calls | [verify](/crates/oxide-app/tests/support/mod/verify.md) |
| called_by | [fixture](/crates/oxide-app/tests/async_library_mount/fixture.md) |
| called_by | [auto_mount_refreshes_an_already_mounted_library](/crates/oxide-app/tests/library_open_cache/auto_mount_refreshes_an_already_mounted_library.md) |
| called_by | [commands_open_library_refreshes_an_already_mounted_library](/crates/oxide-app/tests/library_open_cache/commands_open_library_refreshes_an_already_mounted_library.md) |
| called_by | [open_library_primes_every_cache](/crates/oxide-app/tests/library_open_cache/open_library_primes_every_cache.md) |
| called_by | [refresh_components_after_open_changes_nothing](/crates/oxide-app/tests/library_open_cache/refresh_components_after_open_changes_nothing.md) |
| called_by | [measure_library_open](/crates/oxide-app/tests/measure_library_open/measure_library_open.md) |
