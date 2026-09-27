---
okf_version: "0.2"
type: Function
title: config_root
description: "Resolve the directory oxide's per-user config files live under."
resource: crates/oxide-app/src/config_root.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/config_root/config_root
language: rust
---

# config_root

Resolve the directory oxide's per-user config files live under.

## Signature

```rust
pub fn config_root() -> Option<PathBuf>
```

## Visibility

- `pub`

## Docstring

Resolve the directory oxide's per-user config files live under.

Under the test/dev redirect ([`is_test_redirect_active`]), returns one
shared per-process tempdir instead of the real OS config directory —
`<tmp>/oxide-test-prefs-<pid>/` — so all four config files land under
the same throwaway root during a test run and none of them read or
write the developer's real config directory. Originally
`fonts::prefs_path`-only (#437), hoisted to cover all four resolvers
in #440.

## Source
Lines 49–54 in `crates/oxide-app/src/config_root.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [config_root](/crates/oxide-app/src/config_root.md) |
| calls | [is_test_redirect_active](/crates/oxide-app/src/config_root/is_test_redirect_active.md) |
| calls | [config_root_for_dir](/crates/oxide-app/src/config_root/config_root_for_dir.md) |
| called_by | [resolves_to_the_shared_per_process_tempdir_under_test](/crates/oxide-app/src/config_root/resolves_to_the_shared_per_process_tempdir_under_test.md) |
| called_by | [prefs_path](/crates/oxide-app/src/fonts/mod/prefs_path.md) |
| called_by | [prefs_path_lives_under_the_shared_config_root](/crates/oxide-app/src/fonts/mod/prefs_path_lives_under_the_shared_config_root.md) |
| called_by | [config_path](/crates/oxide-app/src/keymap/profile/config_path.md) |
| called_by | [config_path](/crates/oxide-app/src/library/settings/persistence/config_path.md) |
| called_by | [prefs_path](/crates/oxide-app/src/panels/components_panel/global_prefs/prefs_path.md) |
| called_by | [config_root_resolves_under_the_os_temp_dir_during_tests](/crates/oxide-app/tests/shared_config_root/config_root_resolves_under_the_os_temp_dir_during_tests.md) |
| called_by | [distributors_config_path_lives_under_shared_root](/crates/oxide-app/tests/shared_config_root/distributors_config_path_lives_under_shared_root.md) |
| called_by | [global_libraries_prefs_path_lives_under_shared_root](/crates/oxide-app/tests/shared_config_root/global_libraries_prefs_path_lives_under_shared_root.md) |
| called_by | [keymap_config_path_lives_under_shared_root](/crates/oxide-app/tests/shared_config_root/keymap_config_path_lives_under_shared_root.md) |
