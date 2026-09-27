---
okf_version: "0.2"
type: Function
title: prepare_mount
description: "Open the adapter and prime every cache — the `spawn_blocking` body."
resource: crates/oxide-app/src/library/mount.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/mount/prepare_mount
language: rust
---

# prepare_mount

Open the adapter and prime every cache — the `spawn_blocking` body.

## Signature

```rust
pub fn prepare_mount(path: &Path) -> Result<PreparedMount, String>
```

## Visibility

- `pub`

## Docstring

Open the adapter and prime every cache — the `spawn_blocking` body.

`path` is the `.snxlib` **file** path, not the library root
directory. `OpenLibrary::root` holds the file path despite its name
(`root_dir()` is the parent), and `LocalGitAdapter::open` agrees;
passing a directory fails at runtime through `validate_file_path`
with a `Backend` error, not at compile time.

Errors are stringified because they have to survive the
`Task::perform` boundary and `Message` is `Clone` while
[`LibraryError`] is not — the same reason `read_and_parse_schematic`
returns `Result<_, String>`.

## Source
Lines 143–172 in `crates/oxide-app/src/library/mount.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mount](/crates/oxide-app/src/library/mount.md) |
| calls | [adapter_ref](/crates/oxide-app/src/library/mount/adapter_ref.md) |
| called_by | [prepare_mount_off_thread](/crates/oxide-app/src/library/mount/prepare_mount_off_thread.md) |
| called_by | [mount_prepared_does_not_duplicate_an_already_mounted_library](/crates/oxide-app/tests/async_library_mount/mount_prepared_does_not_duplicate_an_already_mounted_library.md) |
| called_by | [prepare_mount_then_mount_prepared_matches_open_library](/crates/oxide-app/tests/async_library_mount/prepare_mount_then_mount_prepared_matches_open_library.md) |
| called_by | [measure_library_open](/crates/oxide-app/tests/measure_library_open/measure_library_open.md) |
