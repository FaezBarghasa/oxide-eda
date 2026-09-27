---
okf_version: "0.2"
type: Function
title: load_profile_set_at
resource: crates/oxide-app/src/keymap/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile/load_profile_set_at
language: rust
---

# load_profile_set_at

## Signature

```rust
pub fn load_profile_set_at(path: &Path) -> Result<ShortcutProfileSet, ProfileLoadError>
```

## Visibility

- `pub`

## Source
Lines 369–379 in `crates/oxide-app/src/keymap/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-app/src/keymap/profile.md) |
| called_by | [load_profile_set](/crates/oxide-app/src/keymap/profile/load_profile_set.md) |
| called_by | [load_reports_error_when_the_active_profile_id_does_not_resolve](/crates/oxide-app/src/keymap/profile_tests/load_reports_error_when_the_active_profile_id_does_not_resolve.md) |
| called_by | [load_reports_error_when_the_file_cannot_be_parsed](/crates/oxide-app/src/keymap/profile_tests/load_reports_error_when_the_file_cannot_be_parsed.md) |
| called_by | [load_succeeds_when_the_file_is_absent](/crates/oxide-app/src/keymap/profile_tests/load_succeeds_when_the_file_is_absent.md) |
| called_by | [persistence_rejects_custom_profile_shadowing_built_in_id](/crates/oxide-app/src/keymap/profile_tests/persistence_rejects_custom_profile_shadowing_built_in_id.md) |
| called_by | [persistence_round_trip_keeps_bundled_profiles_and_active_custom_profile](/crates/oxide-app/src/keymap/profile_tests/persistence_round_trip_keeps_bundled_profiles_and_active_custom_profile.md) |
