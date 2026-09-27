---
okf_version: "0.2"
type: Module
title: config_root
description: "Shared config-root resolver for oxide's on-disk preference files."
resource: crates/oxide-app/src/config_root.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/config_root
language: rust
---

# config_root

Shared config-root resolver for oxide's on-disk preference files.

## Docstring

Shared config-root resolver for oxide's on-disk preference files.

Four files persist independently under the same OS-native config
directory: `prefs.json` ([`crate::fonts`]), `keyboard_shortcuts.toml`
(`crate::keymap::profile`), `distributors.toml`
(`crate::library::settings::persistence`), and `global_libraries.toml`
(`crate::panels::components_panel::global_prefs`). Each of those
modules used to compute `dirs::config_dir().join("oxide")` itself;
this module hoists that one shared computation — and its test
redirect — so there is a single place that decides *where* oxide's
config directory is. Each file keeps its own name, its own `None`
fallback, and its own error handling (issue #440).

Resolves to:
- Windows: `%APPDATA%\oxide\`
- macOS:   `~/Library/Application Support/oxide/`
- Linux:   `$XDG_CONFIG_HOME/oxide/` (or `~/.config/oxide/`)

`None` when the platform has no config directory to offer at all
(`dirs::config_dir()` returned `None` — rare: a stripped-down sandbox
or CI runner, a daemon with neither `$HOME` nor `$XDG_CONFIG_HOME`).
None of the four callers has a CWD fallback on `None` — each decides
its own behaviour; see their own doc comments.

## Relationships

| Type | Target |
|------|--------|
| related | [is_test_redirect_active](/crates/oxide-app/src/config_root/is_test_redirect_active.md) |
| related | [config_root](/crates/oxide-app/src/config_root/config_root.md) |
| related | [config_root_for_dir](/crates/oxide-app/src/config_root/config_root_for_dir.md) |
| related | [redirect_is_active_under_the_crates_own_test_build](/crates/oxide-app/src/config_root/redirect_is_active_under_the_crates_own_test_build.md) |
| related | [resolves_to_the_shared_per_process_tempdir_under_test](/crates/oxide-app/src/config_root/resolves_to_the_shared_per_process_tempdir_under_test.md) |
