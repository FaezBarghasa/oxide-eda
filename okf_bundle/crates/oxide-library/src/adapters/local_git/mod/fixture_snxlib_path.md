---
okf_version: "0.2"
type: Function
title: fixture_snxlib_path
resource: crates/oxide-library/src/adapters/local_git/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/adapters/local_git/mod/fixture_snxlib_path
language: rust
---

# fixture_snxlib_path

## Signature

```rust
fn fixture_snxlib_path(dir: &tempfile::TempDir, name: &str) -> PathBuf
```

## Source
Lines 372–374 in `crates/oxide-library/src/adapters/local_git/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git](/crates/oxide-library/src/adapters/local_git/mod.md) |
| called_by | [init_creates_snxlib_file_and_repo](/crates/oxide-library/src/adapters/local_git/mod/init_creates_snxlib_file_and_repo.md) |
| called_by | [lfs_off_skips_gitattributes](/crates/oxide-library/src/adapters/local_git/mod/lfs_off_skips_gitattributes.md) |
| called_by | [lfs_opt_in_writes_gitattributes](/crates/oxide-library/src/adapters/local_git/mod/lfs_opt_in_writes_gitattributes.md) |
| called_by | [save_then_load_symbol_round_trip](/crates/oxide-library/src/adapters/local_git/mod/save_then_load_symbol_round_trip.md) |
