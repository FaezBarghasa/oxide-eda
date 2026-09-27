---
okf_version: "0.2"
type: Function
title: materialize_pending_library
description: "Materialise a previously-registered pending library: do the"
resource: crates/oxide-app/src/library/commands.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/commands/materialize_pending_library
language: rust
---

# materialize_pending_library

Materialise a previously-registered pending library: do the

## Signature

```rust
pub fn materialize_pending_library(
    state: &mut LibraryState,
    project: &mut ProjectData,
    library_id: Uuid,
    spec: &PendingLibrarySpec,
) -> Result<(), LibraryError>
```

## Visibility

- `pub`

## Docstring

Materialise a previously-registered pending library: do the
`.snxlib/` directory + manifest + git scaffolding + project
registration, using the pre-minted `library_id` so the on-disk
manifest matches whatever the user already has staged in
`LoadedProject.pending_libraries`.

This is the disk-write side of the deferred flow. Called from
`save_active_project_if_dirty` once per pending entry; on
success the caller drains the entry from the pending map. On
failure the entry stays pending so the user can retry on next
save (e.g. they free up the target path).

Atomic-with-rollback discipline matches the original
`create_library_at` (single library fn, never split across app +
lib layers — `feedback_no_disk_writes_without_user_save.md`).

## Source
Lines 161–239 in `crates/oxide-app/src/library/commands.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [commands](/crates/oxide-app/src/library/commands.md) |
| calls | [read_component_classes_pref](/crates/oxide-app/src/fonts/mod/read_component_classes_pref.md) |
| calls | [open_library](/crates/oxide-app/src/library/commands/open_library.md) |
| called_by | [persist_project_by_id](/crates/oxide-app/src/app/handlers/document_files/save/persist_project_by_id.md) |
