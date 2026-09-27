---
okf_version: "0.2"
type: Module
title: support
description: "Synthetic `.snxlib` generator, shared by the library-open test targets:"
resource: crates/oxide-app/tests/support/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/support/mod
language: rust
---

# support

Synthetic `.snxlib` generator, shared by the library-open test targets:

## Docstring

Synthetic `.snxlib` generator, shared by the library-open test targets:
`measure_library_open.rs` (timing probe) and `library_open_cache.rs`
(cache-priming regression). Each target `mod support;`s its own copy —
hence the blanket `allow(dead_code)`, since neither uses all of it.

Everything on disk is produced by the *real* Oxide writers —
`LocalGitAdapter::init` for the manifest, `SymbolFile::to_toml_string`
/ `FootprintFile::to_toml_string` / `SimFile::to_toml_string` for the
primitive envelopes, `LocalGitAdapter::insert_row` for the component
rows, and `LocalGitAdapter::recover_init` to land the whole tree in one
git commit. No file layout or wire format is hand-rolled here.

Why not `save_symbol` / `save_footprint` per primitive: each of those
does `scan_symbol_files` (O(n) re-parse of the whole library) plus a
git commit, so generating 2000 symbols that way is O(n²) and would take
longer than the entire measurement. The bytes on disk are identical —
`save_symbol_in_container`'s new-file branch is exactly
`SymbolFile::from_symbol(..).to_toml_string()` + write, and
`write_primitive`'s not-yet-exists branch is exactly
`FootprintFile::from_footprint(..).to_toml_string()` + write.

## Relationships

| Type | Target |
|------|--------|
| related | [Scale](/crates/oxide-app/tests/support/mod/Scale.md) |
| related | [new](/crates/oxide-app/tests/support/mod/new.md) |
| related | [new](/crates/oxide-app/tests/support/mod/new.md) |
| related | [sample_params](/crates/oxide-app/tests/support/mod/sample_params.md) |
| related | [make_symbol](/crates/oxide-app/tests/support/mod/make_symbol.md) |
| related | [make_footprint](/crates/oxide-app/tests/support/mod/make_footprint.md) |
| related | [make_sim](/crates/oxide-app/tests/support/mod/make_sim.md) |
| related | [make_row](/crates/oxide-app/tests/support/mod/make_row.md) |
| related | [manifest](/crates/oxide-app/tests/support/mod/manifest.md) |
| related | [generate_library](/crates/oxide-app/tests/support/mod/generate_library.md) |
| related | [append_component](/crates/oxide-app/tests/support/mod/append_component.md) |
| related | [verify](/crates/oxide-app/tests/support/mod/verify.md) |
| related | [Sizes](/crates/oxide-app/tests/support/mod/Sizes.md) |
| related | [first_file_len](/crates/oxide-app/tests/support/mod/first_file_len.md) |
| related | [dir_total](/crates/oxide-app/tests/support/mod/dir_total.md) |
| related | [primitive_file_sizes](/crates/oxide-app/tests/support/mod/primitive_file_sizes.md) |
