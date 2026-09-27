---
okf_version: "0.2"
type: Module
title: local_git
description: "Local + git storage adapter — `.snxlib` file backed by libgit2."
resource: crates/oxide-library/src/adapters/local_git/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/adapters/local_git/mod
language: rust
---

# local_git

Local + git storage adapter — `.snxlib` file backed by libgit2.

## Docstring

Local + git storage adapter — `.snxlib` file backed by libgit2.

Per `v0.9-snxlib-as-file-plan.md`, a Oxide library on disk is a
*directory* containing a `.snxlib` file (the user-facing entry
point) and sibling `symbols/` / `footprints/` / `sims/` /
`models/` directories. The `.git/` repo lives at the parent
directory so per-primitive `git log -- symbols/<name>.snxsym`
works line-by-line — that's the load-bearing reason the layout
is multi-file.

```text
mylib/                           (root_dir — git working tree)
├── mylib.snxlib                 (file_path — TOML manifest + [tables.<name>] TSV)
├── symbols/<slug>.snxsym
├── footprints/<uuid>.snxfpt
├── sims/<uuid>.snxsim
├── models/                      (3D models, optionally LFS-tracked)
├── .gitattributes               (written when [`LibraryInitOptions::use_lfs`])
└── .git/
```

Tables (component rows) live *inside* the `.snxlib` file under
`[tables.<name>]` blocks — there are no separate `tables/*.tsv`
files anymore. The trait still talks in [`ComponentRow`] for v0.9
compatibility; the adapter converts between the legacy 16-column
[`crate::tables::TABLE_HEADER`] schema and the new
[`LibraryRow`] cell-map at the boundary. Stage 12 will retire
the legacy schema in favour of user-defined columns.

## Relationships

| Type | Target |
|------|--------|
| related | [LibraryInitOptions](/crates/oxide-library/src/adapters/local_git/mod/LibraryInitOptions.md) |
| related | [LocalGitAdapter](/crates/oxide-library/src/adapters/local_git/mod/LocalGitAdapter.md) |
| related | [init](/crates/oxide-library/src/adapters/local_git/mod/init.md) |
| related | [open](/crates/oxide-library/src/adapters/local_git/mod/open.md) |
| related | [recover_init](/crates/oxide-library/src/adapters/local_git/mod/recover_init.md) |
| related | [validate_file_path](/crates/oxide-library/src/adapters/local_git/mod/validate_file_path.md) |
| related | [root](/crates/oxide-library/src/adapters/local_git/mod/root.md) |
| related | [file_path_buf](/crates/oxide-library/src/adapters/local_git/mod/file_path_buf.md) |
| related | [init](/crates/oxide-library/src/adapters/local_git/mod/init.md) |
| related | [open](/crates/oxide-library/src/adapters/local_git/mod/open.md) |
| related | [recover_init](/crates/oxide-library/src/adapters/local_git/mod/recover_init.md) |
| related | [validate_file_path](/crates/oxide-library/src/adapters/local_git/mod/validate_file_path.md) |
| related | [root](/crates/oxide-library/src/adapters/local_git/mod/root.md) |
| related | [file_path_buf](/crates/oxide-library/src/adapters/local_git/mod/file_path_buf.md) |
| related | [fixture_snx_manifest](/crates/oxide-library/src/adapters/local_git/mod/fixture_snx_manifest.md) |
| related | [fixture_snxlib_path](/crates/oxide-library/src/adapters/local_git/mod/fixture_snxlib_path.md) |
| related | [fixture_symbol](/crates/oxide-library/src/adapters/local_git/mod/fixture_symbol.md) |
| related | [init_creates_snxlib_file_and_repo](/crates/oxide-library/src/adapters/local_git/mod/init_creates_snxlib_file_and_repo.md) |
| related | [save_then_load_symbol_round_trip](/crates/oxide-library/src/adapters/local_git/mod/save_then_load_symbol_round_trip.md) |
| related | [lfs_opt_in_writes_gitattributes](/crates/oxide-library/src/adapters/local_git/mod/lfs_opt_in_writes_gitattributes.md) |
| related | [lfs_off_skips_gitattributes](/crates/oxide-library/src/adapters/local_git/mod/lfs_off_skips_gitattributes.md) |
| related | [rejects_non_snxlib_extension](/crates/oxide-library/src/adapters/local_git/mod/rejects_non_snxlib_extension.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
