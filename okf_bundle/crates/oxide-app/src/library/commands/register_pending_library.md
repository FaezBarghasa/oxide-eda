---
okf_version: "0.2"
type: Function
title: register_pending_library
description: Register a library creation request without touching disk.
resource: crates/oxide-app/src/library/commands.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/commands/register_pending_library
language: rust
---

# register_pending_library

Register a library creation request without touching disk.

## Signature

```rust
pub fn register_pending_library(
    lib_path: PathBuf,
    enable_git: bool,
    use_lfs: bool,
) -> Result<(Uuid, PendingLibrarySpec), LibraryError>
```

## Visibility

- `pub`

## Docstring

Register a library creation request without touching disk.

Validates the target path the same way `create_library_at` does
(extension must be `.snxlib`, stem non-empty + portable, target
must not already exist), pre-mints a `library_id`, and stashes a
[`PendingLibrarySpec`] under that id on the caller-provided
pending map. The actual `.snxlib` directory + manifest +
optional `git init` happen at project-save time via
[`materialize_pending_library`].

Returns the pre-minted `library_id`. Caller is responsible for:
1. Storing the `(library_id, PendingLibrarySpec)` on the
project's `pending_libraries` map.
2. Marking the project dirty so the user knows Save is pending.

## Source
Lines 95–144 in `crates/oxide-app/src/library/commands.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [commands](/crates/oxide-app/src/library/commands.md) |
| called_by | [handle_create_library_at_path](/crates/oxide-app/src/app/dispatch/library/registration/handle_create_library_at_path.md) |
| called_by | [f13_register_pending_library_does_not_touch_disk](/crates/oxide-app/tests/regression/project/f13_register_pending_library_does_not_touch_disk.md) |
| called_by | [f13_register_pending_rejects_existing_path](/crates/oxide-app/tests/regression/project/f13_register_pending_rejects_existing_path.md) |
| called_by | [f13_register_pending_rejects_non_snxlib_extension](/crates/oxide-app/tests/regression/project/f13_register_pending_rejects_non_snxlib_extension.md) |
