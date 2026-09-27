---
okf_version: "0.2"
type: Function
title: serial
description: "A poisoned lock is not a reason to fail every remaining test: the guard"
resource: crates/oxide-app/tests/regression/preferences_prefs_recovery.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/preferences_prefs_recovery/serial
language: rust
---

# serial

A poisoned lock is not a reason to fail every remaining test: the guard

## Signature

```rust
fn serial() -> std::sync::MutexGuard<'static, ()>
```

## Docstring

A poisoned lock is not a reason to fail every remaining test: the guard
protects a file path, not an invariant that a panicking test can leave
half-applied, and [`PrefsPathGuard`] has already restored the path by
the time this lock is released.

## Source
Lines 55–59 in `crates/oxide-app/tests/regression/preferences_prefs_recovery.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences_prefs_recovery](/crates/oxide-app/tests/regression/preferences_prefs_recovery.md) |
| called_by | [opening_preferences_reprobes_the_prefs_file](/crates/oxide-app/tests/regression/preferences_prefs_recovery/opening_preferences_reprobes_the_prefs_file.md) |
| called_by | [reopening_preferences_clears_the_reset_status](/crates/oxide-app/tests/regression/preferences_prefs_recovery/reopening_preferences_clears_the_reset_status.md) |
| called_by | [reporting_the_reset_outcome_does_not_mark_the_dialog_dirty](/crates/oxide-app/tests/regression/preferences_prefs_recovery/reporting_the_reset_outcome_does_not_mark_the_dialog_dirty.md) |
| called_by | [resetting_a_healthy_prefs_file_moves_nothing_and_says_so](/crates/oxide-app/tests/regression/preferences_prefs_recovery/resetting_a_healthy_prefs_file_moves_nothing_and_says_so.md) |
| called_by | [resetting_clears_a_stale_load_error_flag](/crates/oxide-app/tests/regression/preferences_prefs_recovery/resetting_clears_a_stale_load_error_flag.md) |
| called_by | [saving_reprobes_a_file_that_broke_while_the_dialog_was_open](/crates/oxide-app/tests/regression/preferences_prefs_recovery/saving_reprobes_a_file_that_broke_while_the_dialog_was_open.md) |
