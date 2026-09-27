---
okf_version: "0.2"
type: Function
title: enable_project_version_control
description: "Project-level \"Enable Version Control\" helper. Runs"
resource: crates/oxide-library/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:59:15Z"
concept_id: crates/oxide-library/src/lib/enable_project_version_control
language: rust
---

# enable_project_version_control

Project-level "Enable Version Control" helper. Runs

## Signature

```rust
pub fn enable_project_version_control(
    project_dir: &std::path::Path,
    use_lfs: bool,
    gitignore: Option<&str>,
) -> Result<(), adapter::LibraryError>
```

## Decorators

- `cfg(feature = "local-git")`

## Visibility

- `pub`

## Docstring

Project-level "Enable Version Control" helper. Runs
`git2::Repository::init` at `project_dir`, optionally writes a
`.gitattributes` opting common binary-model extensions (`*.step`,
`*.stp`, `*.wrl`, `*.iges`) into Git LFS, optionally writes a
`.gitignore` (`gitignore` arg) for any items the per-project
tracking-scope picker unchecked, then stages every tracked file
and creates the initial commit "chore: enable version control".

Used by `oxide-app` so the per-project Enable Version Control
flow doesn't need to pull `git2` in directly. Errors propagate
through the existing [`adapter::LibraryError`] variants so the UI
can surface them in one place.
[cfg(feature = "local-git")]

## Source
Lines 124–238 in `crates/oxide-library/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-library/src/lib.md) |
| calls | [cleanup](/crates/oxide-library/tests/distributor_keyring/cleanup.md) |
| called_by | [try_init_project_repo](/crates/oxide-app/src/app/handlers/dock/project_navigation/version_control/try_init_project_repo.md) |
