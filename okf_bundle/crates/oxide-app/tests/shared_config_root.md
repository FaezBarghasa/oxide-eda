---
okf_version: "0.2"
type: Module
title: shared_config_root
description: "Guard (#440): the four independent on-disk prefs resolvers —"
resource: crates/oxide-app/tests/shared_config_root.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/shared_config_root
language: rust
---

# shared_config_root

Guard (#440): the four independent on-disk prefs resolvers —

## Docstring

Guard (#440): the four independent on-disk prefs resolvers —
`fonts::prefs_path()`, `keymap::config_path()`,
`library::settings::persistence::config_path()`, and
`panels::components_panel::global_prefs::prefs_path()` — must all
resolve under the one shared `config_root::config_root()`, and that
root itself must resolve under the OS temp dir during a test run
(never the developer's real config directory).

The #440 branch that hoisted `config_root()` proved this by hand
with scratch tests deleted before commit, so nothing in the
committed tree failed if a fifth resolver bypassed `config_root()`,
or if one of the four was repointed back at `dirs::config_dir()`
directly. This file is the permanent replacement. `fonts::
prefs_path()` is private and unreachable here — its half of the
guard is the unit test in `src/fonts/mod.rs`.

Discriminates: reverting `keymap/profile.rs`'s `config_path()` to
`dirs::config_dir().map(|b| config_path_for_dir(&b))` makes
`keymap_config_path_lives_under_shared_root` fail (it resolves
outside the temp-dir root, or panics `Oxide settings header`-free
against the developer's real config dir) — see the commit message
for the red/green run.

## Relationships

| Type | Target |
|------|--------|
| related | [config_root_resolves_under_the_os_temp_dir_during_tests](/crates/oxide-app/tests/shared_config_root/config_root_resolves_under_the_os_temp_dir_during_tests.md) |
| related | [keymap_config_path_lives_under_shared_root](/crates/oxide-app/tests/shared_config_root/keymap_config_path_lives_under_shared_root.md) |
| related | [distributors_config_path_lives_under_shared_root](/crates/oxide-app/tests/shared_config_root/distributors_config_path_lives_under_shared_root.md) |
| related | [global_libraries_prefs_path_lives_under_shared_root](/crates/oxide-app/tests/shared_config_root/global_libraries_prefs_path_lives_under_shared_root.md) |
