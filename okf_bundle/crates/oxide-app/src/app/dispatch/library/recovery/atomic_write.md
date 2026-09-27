---
okf_version: "0.2"
type: Function
title: atomic_write
description: "Atomic write — write `bytes` to `<path>.tmp` then `rename` over"
resource: crates/oxide-app/src/app/dispatch/library/recovery.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/recovery/atomic_write
language: rust
---

# atomic_write

Atomic write — write `bytes` to `<path>.tmp` then `rename` over

## Signature

```rust
pub(super) fn atomic_write(path: &std::path::Path, bytes: &[u8]) -> std::io::Result<()>
```

## Visibility

- `pub(super)`

## Docstring

Atomic write — write `bytes` to `<path>.tmp` then `rename` over
`path`. A crash mid-write leaves either the original file intact
Re-export of the shared atomic-write helper (HI-6). Lives in
`oxide-types::atomic_io` so engine, library, and app share one
implementation; the function used to be a private duplicate here.

## Source
Lines 15–17 in `crates/oxide-app/src/app/dispatch/library/recovery.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [recovery](/crates/oxide-app/src/app/dispatch/library/recovery.md) |
| called_by | [save_primitive_tab_at](/crates/oxide-app/src/app/dispatch/library/editor/save_primitive_tab_at.md) |
| called_by | [handle_add_library_footprint_file_picked](/crates/oxide-app/src/app/dispatch/library/registration/handle_add_library_footprint_file_picked.md) |
| called_by | [handle_add_library_symbol_file_picked](/crates/oxide-app/src/app/dispatch/library/registration/handle_add_library_symbol_file_picked.md) |
| called_by | [handle_add_new_schematic_picked](/crates/oxide-app/src/app/handlers/dock/project_navigation/add/handle_add_new_schematic_picked.md) |
| called_by | [create_new_project](/crates/oxide-app/src/app/handlers/document_files/open/create_new_project.md) |
| called_by | [write_pref_atomic](/crates/oxide-app/src/fonts/mod/write_pref_atomic.md) |
| called_by | [move_prefs_file_aside_at](/crates/oxide-app/src/fonts/prefs_file/move_prefs_file_aside_at.md) |
| called_by | [save_profile_set_at](/crates/oxide-app/src/keymap/profile/save_profile_set_at.md) |
| called_by | [stash_step](/crates/oxide-app/src/library/editor/footprint/step_attach/stash_step.md) |
| called_by | [save_preferred_order_at](/crates/oxide-app/src/library/settings/persistence/save_preferred_order_at.md) |
| called_by | [save_at](/crates/oxide-app/src/panels/components_panel/global_prefs/save_at.md) |
