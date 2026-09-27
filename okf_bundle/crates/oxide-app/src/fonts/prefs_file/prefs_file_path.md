---
okf_version: "0.2"
type: Function
title: prefs_file_path
description: "The resolved `prefs.json` path, for UI that has to *name* the file."
resource: crates/oxide-app/src/fonts/prefs_file.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/prefs_file/prefs_file_path
language: rust
---

# prefs_file_path

The resolved `prefs.json` path, for UI that has to *name* the file.

## Signature

```rust
pub fn prefs_file_path() -> PathBuf
```

## Visibility

- `pub`

## Docstring

The resolved `prefs.json` path, for UI that has to *name* the file.

[`super::prefs_path`] is private to `fonts` and this is its child
module, so this is the only way the Preferences banner can name the
real file rather than re-deriving a path that could disagree with the
one actually written (the no-config-dir fallback is a per-process
temp directory, which nothing outside this module can reconstruct).

## Source
Lines 203–205 in `crates/oxide-app/src/fonts/prefs_file.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs_file](/crates/oxide-app/src/fonts/prefs_file.md) |
| called_by | [check_prefs_file](/crates/oxide-app/src/fonts/prefs_file/check_prefs_file.md) |
| called_by | [move_prefs_file_aside](/crates/oxide-app/src/fonts/prefs_file/move_prefs_file_aside.md) |
| called_by | [build_prefs_file_banner](/crates/oxide-app/src/preferences/mod/build_prefs_file_banner.md) |
| called_by | [capture](/crates/oxide-app/tests/regression/preferences_prefs_recovery/capture.md) |
