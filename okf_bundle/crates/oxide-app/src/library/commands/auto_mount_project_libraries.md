---
okf_version: "0.2"
type: Function
title: auto_mount_project_libraries
description: "Auto-mount every library referenced by `project.libraries`. Called"
resource: crates/oxide-app/src/library/commands.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/commands/auto_mount_project_libraries
language: rust
---

# auto_mount_project_libraries

Auto-mount every library referenced by `project.libraries`. Called

## Signature

```rust
pub fn auto_mount_project_libraries(
    state: &mut LibraryState,
    project: &ProjectData,
) -> AutoMountOutcome
```

## Visibility

- `pub`

## Docstring

Auto-mount every library referenced by `project.libraries`. Called
once when a project loads. Failures are logged and skipped — a
missing or corrupt library shouldn't block the rest of the project
from opening.

**Cold mounts are prepared off the UI thread** (#99 part 2c): this
function records them and returns their paths, and the caller fans
them out. Six libraries then parse in parallel instead of serially
inside `update()` — the measured cost was 826.229 ms after #528, about
50 dropped frames, and a single mount already crosses one 60 Hz frame
at ~58 symbols + 58 footprints.

The `refresh_components` chaser still runs on the warm path **only**.
On the cold path the caches are primed by `mount::prepare_mount`
(off-thread) exactly as [`LibraryState::open_library`] primed them
inline before, so re-running it would be the same pure duplicate work
#528 removed: it roughly doubled every project open, costing a
six-medium-library project 795.7 ms of its 1 622.571 ms mount.
`tests/library_open_cache.rs` pins both halves — that the cold path
primes everything, and that the warm path still rescans the primitive
directories.

## Source
Lines 462–547 in `crates/oxide-app/src/library/commands.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [commands](/crates/oxide-app/src/library/commands.md) |
| called_by | [load_or_activate_project](/crates/oxide-app/src/app/handlers/document_files/open/load_or_activate_project.md) |
| called_by | [auto_mount_refreshes_an_already_mounted_library](/crates/oxide-app/tests/library_open_cache/auto_mount_refreshes_an_already_mounted_library.md) |
| called_by | [measure_library_open](/crates/oxide-app/tests/measure_library_open/measure_library_open.md) |
