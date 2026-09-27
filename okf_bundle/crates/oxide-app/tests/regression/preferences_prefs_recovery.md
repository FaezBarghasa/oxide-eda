---
okf_version: "0.2"
type: Module
title: preferences_prefs_recovery
description: "#602 — the Preferences prefs-file banner and its recovery action must"
resource: crates/oxide-app/tests/regression/preferences_prefs_recovery.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/preferences_prefs_recovery
language: rust
---

# preferences_prefs_recovery

#602 — the Preferences prefs-file banner and its recovery action must

## Docstring

#602 — the Preferences prefs-file banner and its recovery action must
actually be wired to the handler.

`crates/oxide-app/src/fonts/prefs_file.rs` owns the filesystem
behaviour (rename, `.bak` laddering, the `{}` re-seed, writes resuming)
and tests it hermetically against tempdirs. What no test covered
anywhere was the wiring: opening Preferences re-probes the file, saving
re-probes it again, and `PrefMsg::ResetPrefsFile` reaches the handler at
all. The keymap side left the same gap — `keymap/profile_tests.rs`
re-enacts its handler by hand rather than driving
`app/handlers/preferences.rs`.

**Sharing the prefs path, safely.** Integration tests run under the
`test-prefs-redirect` feature, so `config_root()` is ONE per-process
tempdir shared by every test in this binary, and `prefs_path()` caches
it in a `OnceLock`. Two tests here have to seed a broken file at that
shared path, because "the flag gets SET" is the direction that can
actually fail — a test that only ever sees a healthy file also passes
against an unconditional `prefs_load_error = None`, which would delete
the whole feature and leave the suite green.

That is safe for two reasons, and both were checked rather than
assumed:

1. Every test in this module takes [`serial`] first, so no two of them
disagree about what is at the shared path, and [`PrefsPathGuard`]
puts it back on every exit path including a panicking assertion.
2. No other registered test in this binary depends on that file. Every
prefs test uses the `_at(tempdir)` variants
(`tests/regression/prefs.rs`), no test drives `PrefMsg::Save`, and
no test asserts on the shared prefs bytes. A reader that hits the
seeded broken file falls back to exactly the defaults it already
gets from the absent file, so no assertion anywhere changes.

## Relationships

| Type | Target |
|------|--------|
| related | [inner](/crates/oxide-app/tests/regression/preferences_prefs_recovery/inner.md) |
| related | [serial](/crates/oxide-app/tests/regression/preferences_prefs_recovery/serial.md) |
| related | [PrefsPathGuard](/crates/oxide-app/tests/regression/preferences_prefs_recovery/PrefsPathGuard.md) |
| related | [capture](/crates/oxide-app/tests/regression/preferences_prefs_recovery/capture.md) |
| related | [seed_broken](/crates/oxide-app/tests/regression/preferences_prefs_recovery/seed_broken.md) |
| related | [capture](/crates/oxide-app/tests/regression/preferences_prefs_recovery/capture.md) |
| related | [seed_broken](/crates/oxide-app/tests/regression/preferences_prefs_recovery/seed_broken.md) |
| related | [restore](/crates/oxide-app/tests/regression/preferences_prefs_recovery/restore.md) |
| related | [drop](/crates/oxide-app/tests/regression/preferences_prefs_recovery/drop.md) |
| related | [drop](/crates/oxide-app/tests/regression/preferences_prefs_recovery/drop.md) |
| related | [resetting_a_healthy_prefs_file_moves_nothing_and_says_so](/crates/oxide-app/tests/regression/preferences_prefs_recovery/resetting_a_healthy_prefs_file_moves_nothing_and_says_so.md) |
| related | [resetting_clears_a_stale_load_error_flag](/crates/oxide-app/tests/regression/preferences_prefs_recovery/resetting_clears_a_stale_load_error_flag.md) |
| related | [opening_preferences_reprobes_the_prefs_file](/crates/oxide-app/tests/regression/preferences_prefs_recovery/opening_preferences_reprobes_the_prefs_file.md) |
| related | [saving_reprobes_a_file_that_broke_while_the_dialog_was_open](/crates/oxide-app/tests/regression/preferences_prefs_recovery/saving_reprobes_a_file_that_broke_while_the_dialog_was_open.md) |
| related | [reporting_the_reset_outcome_does_not_mark_the_dialog_dirty](/crates/oxide-app/tests/regression/preferences_prefs_recovery/reporting_the_reset_outcome_does_not_mark_the_dialog_dirty.md) |
| related | [reopening_preferences_clears_the_reset_status](/crates/oxide-app/tests/regression/preferences_prefs_recovery/reopening_preferences_clears_the_reset_status.md) |
