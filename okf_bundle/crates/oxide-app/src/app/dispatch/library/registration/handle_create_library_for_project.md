---
okf_version: "0.2"
type: Function
title: handle_create_library_for_project
description: "Spawn the \"New Component Library\" Save-As dialog for the"
resource: crates/oxide-app/src/app/dispatch/library/registration.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/registration/handle_create_library_for_project
language: rust
---

# handle_create_library_for_project

Spawn the "New Component Library" Save-As dialog for the

## Signature

```rust
impl Oxide { pub(super) fn handle_create_library_for_project(
        &mut self,
        project_root: std::path::PathBuf,
    ) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Docstring

Spawn the "New Component Library" Save-As dialog for the
project rooted at `project_root`. The dialog defaults to
`<project_dir>/<project>-lib.snxlib` so the common
project-local case is one Enter key, but the user can navigate
to a global directory to create a shared library. On confirm,
the dialog dispatches `CreateLibraryAtPath` which calls
`commands::create_library_at` to do the actual disk + manifest
+ git init.

We deliberately do NOT touch disk here — the previous "instant
create on click" behaviour was confusing because users
couldn't see where it was going to land or override the
default name.

## Source
Lines 87–151 in `crates/oxide-app/src/app/dispatch/library/registration.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [registration](/crates/oxide-app/src/app/dispatch/library/registration.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
