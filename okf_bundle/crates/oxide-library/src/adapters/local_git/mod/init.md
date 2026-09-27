---
okf_version: "0.2"
type: Function
title: init
description: "Initialise a fresh library at `file_path`. The path must end"
resource: crates/oxide-library/src/adapters/local_git/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/adapters/local_git/mod/init
language: rust
---

# init

Initialise a fresh library at `file_path`. The path must end

## Signature

```rust
impl LocalGitAdapter { pub fn init(
        file_path: impl AsRef<Path>,
        snx_manifest: SnxlibManifest,
        opts: LibraryInitOptions,
    ) -> Result<Self, LibraryError> }
```

## Visibility

- `pub`

## Docstring

Initialise a fresh library at `file_path`. The path must end
in `.snxlib`; its parent directory becomes the git working tree.

Fails with `Conflict` if a `.snxlib` already exists at
`file_path`. Creates the parent directory if it doesn't
already exist.

## Source
Lines 116–198 in `crates/oxide-library/src/adapters/local_git/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git](/crates/oxide-library/src/adapters/local_git/mod.md) |
| calls | [parent_dir](/crates/oxide-library/src/adapters/local_git/helpers/parent_dir.md) |
| calls | [write_lfs_attributes](/crates/oxide-library/src/adapters/local_git/helpers/write_lfs_attributes.md) |
| calls | [file_name_str](/crates/oxide-library/src/adapters/local_git/helpers/file_name_str.md) |
| calls | [add_path](/crates/oxide-app/src/panels/components_panel/global_prefs/add_path.md) |
| calls | [synthesize_manifest](/crates/oxide-library/src/adapters/local_git/helpers/synthesize_manifest.md) |
