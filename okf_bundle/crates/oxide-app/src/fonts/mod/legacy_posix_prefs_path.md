---
okf_version: "0.2"
type: Function
title: legacy_posix_prefs_path
description: "Pre-v0.12 prefs path: POSIX-style under `$XDG_CONFIG_HOME` or"
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod/legacy_posix_prefs_path
language: rust
---

# legacy_posix_prefs_path

Pre-v0.12 prefs path: POSIX-style under `$XDG_CONFIG_HOME` or

## Signature

```rust
fn legacy_posix_prefs_path() -> PathBuf
```

## Docstring

Pre-v0.12 prefs path: POSIX-style under `$XDG_CONFIG_HOME` or
`$HOME/.config`. Only used by [`migrate_legacy_prefs`] to find
existing user files for one-shot migration.

## Source
Lines 367–375 in `crates/oxide-app/src/fonts/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fonts](/crates/oxide-app/src/fonts/mod.md) |
| called_by | [prefs_path](/crates/oxide-app/src/fonts/mod/prefs_path.md) |
