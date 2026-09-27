---
okf_version: "0.2"
type: Function
title: migrate_legacy_prefs
description: "One-shot startup migrations applied to `prefs.json` before any"
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod/migrate_legacy_prefs
language: rust
---

# migrate_legacy_prefs

One-shot startup migrations applied to `prefs.json` before any

## Signature

```rust
pub fn migrate_legacy_prefs(canonical: &Path, legacy: &Path)
```

## Visibility

- `pub`

## Docstring

One-shot startup migrations applied to `prefs.json` before any
reader sees the file. Idempotent — runs at most once per process
because [`prefs_path`] caches via `OnceLock`.

**F1** (Windows prefs path bug): pre-v0.12 the path was hardcoded
to `$XDG_CONFIG_HOME/oxide/prefs.json` or
`$HOME/.config/oxide/prefs.json`. On Windows that landed in a
`.config` subfolder of the user dir rather than the canonical
`%APPDATA%\oxide\`. If the legacy path has a file but the
canonical path doesn't, copy it forward.

**F3** (stale label-style discriminants): pre-v0.10 prefs files
carried label-style tokens that aren't in the current canonical
set. The reader silently falls through to `LabelStyle::Standard`
for unknown discriminants, but the literal stale string lingers
in user-space `prefs.json` until the user changes label style +
saves. Rewrite once on startup so unknown tokens normalise to
the canonical default.

`legacy` is the legacy (pre-v0.12) POSIX-shaped prefs path to
pull from when canonical is missing. Production passes
[`legacy_posix_prefs_path()`]; tests inject a tempdir-shaped path.

## Source
Lines 319–362 in `crates/oxide-app/src/fonts/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fonts](/crates/oxide-app/src/fonts/mod.md) |
| calls | [write_pref_atomic](/crates/oxide-app/src/fonts/mod/write_pref_atomic.md) |
| called_by | [prefs_path](/crates/oxide-app/src/fonts/mod/prefs_path.md) |
| called_by | [moving_aside_is_not_undone_by_the_legacy_prefs_migration](/crates/oxide-app/src/fonts/prefs_file/moving_aside_is_not_undone_by_the_legacy_prefs_migration.md) |
| called_by | [f1_canonical_present_blocks_legacy_copy](/crates/oxide-app/tests/regression/prefs/f1_canonical_present_blocks_legacy_copy.md) |
| called_by | [f1_legacy_prefs_path_copied_forward_when_canonical_empty](/crates/oxide-app/tests/regression/prefs/f1_legacy_prefs_path_copied_forward_when_canonical_empty.md) |
| called_by | [f1_no_legacy_no_canonical_is_a_clean_noop](/crates/oxide-app/tests/regression/prefs/f1_no_legacy_no_canonical_is_a_clean_noop.md) |
| called_by | [f3_canonical_label_style_left_alone](/crates/oxide-app/tests/regression/prefs/f3_canonical_label_style_left_alone.md) |
| called_by | [f3_garbage_json_doesnt_corrupt_file](/crates/oxide-app/tests/regression/prefs/f3_garbage_json_doesnt_corrupt_file.md) |
| called_by | [f3_label_style_case_variants_all_normalise](/crates/oxide-app/tests/regression/prefs/f3_label_style_case_variants_all_normalise.md) |
| called_by | [f3_stale_label_style_rewritten_to_standard](/crates/oxide-app/tests/regression/prefs/f3_stale_label_style_rewritten_to_standard.md) |
