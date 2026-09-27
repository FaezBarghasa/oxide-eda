---
okf_version: "0.2"
type: Function
title: append_component
description: "Append one component — a symbol, a footprint and the row tying them"
resource: crates/oxide-app/tests/support/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/support/mod/append_component
language: rust
---

# append_component

Append one component — a symbol, a footprint and the row tying them

## Signature

```rust
pub fn append_component(snxlib: &Path, index: usize) -> Result<String, LibraryError>
```

## Visibility

- `pub`

## Docstring

Append one component — a symbol, a footprint and the row tying them
together — to an already-generated library, writing straight to disk
through a fresh adapter.

This is the out-of-band edit: another Oxide window, a `git pull`, a
colleague's commit. Nothing in the caller's `LibraryState` knows it
happened, which is exactly the point — it is what the warm
(already-mounted) auto-mount path has to notice.

`index` must not collide with the indices `generate_library` already
wrote, so pass something >= `scale.symbols`. Returns the new row's
internal PN so the caller can assert on it by name.

## Source
Lines 443–479 in `crates/oxide-app/tests/support/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [support](/crates/oxide-app/tests/support/mod.md) |
| calls | [library_id](/crates/oxide-library/src/adapter/library_id.md) |
| calls | [make_symbol](/crates/oxide-app/tests/support/mod/make_symbol.md) |
| calls | [make_footprint](/crates/oxide-app/tests/support/mod/make_footprint.md) |
| calls | [make_row](/crates/oxide-app/tests/support/mod/make_row.md) |
| called_by | [auto_mount_refreshes_an_already_mounted_library](/crates/oxide-app/tests/library_open_cache/auto_mount_refreshes_an_already_mounted_library.md) |
| called_by | [commands_open_library_refreshes_an_already_mounted_library](/crates/oxide-app/tests/library_open_cache/commands_open_library_refreshes_an_already_mounted_library.md) |
