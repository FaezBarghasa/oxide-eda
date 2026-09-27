---
okf_version: "0.2"
type: Function
title: create_library_at
description: "Create a fresh `.snxlib/` library at `lib_path`. The directory's"
resource: crates/oxide-app/src/library/commands.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/commands/create_library_at
language: rust
---

# create_library_at

Create a fresh `.snxlib/` library at `lib_path`. The directory's

## Signature

```rust
pub fn create_library_at(
    state: &mut LibraryState,
    project: &mut ProjectData,
    lib_path: PathBuf,
    enable_git: bool,
    use_lfs: bool,
) -> Result<Uuid, LibraryError>
```

## Visibility

- `pub`

## Docstring

Create a fresh `.snxlib/` library at `lib_path`. The directory's
final filename stem (`<name>.snxlib`) becomes the library's
display name in the manifest. The library is registered on
`project.libraries` as `ProjectLocal` when `lib_path` lives
inside `project.dir`, otherwise `Shared` — so the same call site
handles both the right-click "Add New ▸ Component Library"
project-local case and "save my new symbol into a global library
directory" shared case.

`use_lfs` (Stage 11 of `v0.9-snxlib-as-file-plan.md`) controls
whether `LocalGitAdapter::init` writes a `.gitattributes` opting
`*.step` / `*.stp` / `*.wrl` / `*.iges` into Git LFS at create
time. The library-create UI surfaces this through the "Library
Options" modal that pops up after the Save-As dialog; non-UI
callers (tests, fixtures) pass `false` to stay independent of a
local `git lfs` install.

## Source
Lines 257–395 in `crates/oxide-app/src/library/commands.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [commands](/crates/oxide-app/src/library/commands.md) |
| calls | [read_component_classes_pref](/crates/oxide-app/src/fonts/mod/read_component_classes_pref.md) |
| calls | [open_library](/crates/oxide-app/src/library/commands/open_library.md) |
| called_by | [create_library](/crates/oxide-app/src/library/commands/create_library.md) |
| called_by | [create_library_at_mounts_with_primed_empty_caches](/crates/oxide-app/tests/library_open_cache/create_library_at_mounts_with_primed_empty_caches.md) |
