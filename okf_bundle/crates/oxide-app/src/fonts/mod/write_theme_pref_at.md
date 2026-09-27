---
okf_version: "0.2"
type: Function
title: write_theme_pref_at
description: "Same as [`write_theme_pref`] but writes to `path` — exposed for tests."
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod/write_theme_pref_at
language: rust
---

# write_theme_pref_at

Same as [`write_theme_pref`] but writes to `path` — exposed for tests.

## Signature

```rust
pub fn write_theme_pref_at(path: &Path, theme: ThemeId)
```

## Visibility

- `pub`

## Docstring

Same as [`write_theme_pref`] but writes to `path` — exposed for tests.

## Source
Lines 631–637 in `crates/oxide-app/src/fonts/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fonts](/crates/oxide-app/src/fonts/mod.md) |
| calls | [update_prefs_json](/crates/oxide-app/src/fonts/prefs_file/update_prefs_json.md) |
| called_by | [write_theme_pref](/crates/oxide-app/src/fonts/mod/write_theme_pref.md) |
| called_by | [prefs_cross_pref_independence](/crates/oxide-app/tests/regression/prefs/prefs_cross_pref_independence.md) |
| called_by | [prefs_theme_round_trip_through_json](/crates/oxide-app/tests/regression/prefs/prefs_theme_round_trip_through_json.md) |
| called_by | [prefs_writes_dont_clobber_neighboring_keys](/crates/oxide-app/tests/regression/prefs/prefs_writes_dont_clobber_neighboring_keys.md) |
