---
okf_version: "0.2"
type: Function
title: handle_create_library_at_path
description: "Resolution of the \"Library Options\" modal (Stage 11 of"
resource: crates/oxide-app/src/app/dispatch/library/registration.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/registration/handle_create_library_at_path
language: rust
---

# handle_create_library_at_path

Resolution of the "Library Options" modal (Stage 11 of

## Signature

```rust
impl Oxide { pub(super) fn handle_create_library_at_path(
        &mut self,
        project_path: std::path::PathBuf,
        lib_path: std::path::PathBuf,
        enable_git: bool,
        use_lfs: bool,
    ) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Docstring

Resolution of the "Library Options" modal (Stage 11 of
`v0.9-snxlib-as-file-plan.md`). Re-resolves the project (in
case it was unloaded between modal spawn + confirm), then
**registers** a pending library — no disk writes here. The
actual `.snxlib/` directory + manifest + git scaffolding land
at project-save time via
`commands::materialize_pending_library`, called from
`save_active_project_if_dirty`. Closes
`feedback_no_disk_writes_without_user_save.md`'s "wait for
explicit user save" invariant. `use_lfs` carries the modal's
checkbox state — when on, the eventual adapter writes
`.gitattributes` for `*.step` / `*.stp` / `*.wrl` / `*.iges`
and stages it into the initial commit.

## Source
Lines 166–221 in `crates/oxide-app/src/app/dispatch/library/registration.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [registration](/crates/oxide-app/src/app/dispatch/library/registration.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [register_pending_library](/crates/oxide-app/src/library/commands/register_pending_library.md) |
