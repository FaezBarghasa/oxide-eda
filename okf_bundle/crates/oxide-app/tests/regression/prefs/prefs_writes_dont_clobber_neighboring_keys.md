---
okf_version: "0.2"
type: Function
title: prefs_writes_dont_clobber_neighboring_keys
description: "[test]"
resource: crates/oxide-app/tests/regression/prefs.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/prefs/prefs_writes_dont_clobber_neighboring_keys
language: rust
---

# prefs_writes_dont_clobber_neighboring_keys

[test]

## Signature

```rust
fn prefs_writes_dont_clobber_neighboring_keys()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 283–298 in `crates/oxide-app/tests/regression/prefs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs](/crates/oxide-app/tests/regression/prefs.md) |
| calls | [temp_prefs_path](/crates/oxide-app/tests/regression/prefs/temp_prefs_path.md) |
| calls | [write_theme_pref_at](/crates/oxide-app/src/fonts/mod/write_theme_pref_at.md) |
| calls | [write_unit_pref_at](/crates/oxide-app/src/fonts/mod/write_unit_pref_at.md) |
| calls | [write_grid_visible_pref_at](/crates/oxide-app/src/fonts/mod/write_grid_visible_pref_at.md) |
| calls | [write_snap_enabled_pref_at](/crates/oxide-app/src/fonts/mod/write_snap_enabled_pref_at.md) |
