---
okf_version: "0.2"
type: Function
title: moving_aside_is_not_undone_by_the_legacy_prefs_migration
description: Leaving the prefs path absent handed the reset straight back to
resource: crates/oxide-app/src/fonts/prefs_file.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/prefs_file/moving_aside_is_not_undone_by_the_legacy_prefs_migration
language: rust
---

# moving_aside_is_not_undone_by_the_legacy_prefs_migration

Leaving the prefs path absent handed the reset straight back to

## Signature

```rust
fn moving_aside_is_not_undone_by_the_legacy_prefs_migration()
```

## Decorators

- `test`

## Docstring

Leaving the prefs path absent handed the reset straight back to
`migrate_legacy_prefs`, whose F1 branch fires on exactly
`!canonical.exists()` and runs on the first prefs touch of every
launch. On macOS and Windows the legacy path differs from the
canonical one, so the broken file the user just moved aside would
be copied back over their fresh start — banner, reset, restart,
banner, one `.bak` slot burnt per cycle.
[test]

## Source
Lines 733–758 in `crates/oxide-app/src/fonts/prefs_file.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs_file](/crates/oxide-app/src/fonts/prefs_file.md) |
| calls | [temp_prefs](/crates/oxide-app/src/fonts/prefs_file/temp_prefs.md) |
| calls | [move_prefs_file_aside_at](/crates/oxide-app/src/fonts/prefs_file/move_prefs_file_aside_at.md) |
| calls | [migrate_legacy_prefs](/crates/oxide-app/src/fonts/mod/migrate_legacy_prefs.md) |
