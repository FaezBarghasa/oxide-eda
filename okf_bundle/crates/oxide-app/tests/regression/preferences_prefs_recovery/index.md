# preferences_prefs_recovery

## Classs

- [PrefsPathGuard](PrefsPathGuard.md) — Snapshots the shared config files these tests can disturb and puts

## Functions

- [capture](capture.md)
- [capture](capture_1.md)
- [drop](drop.md)
- [drop](drop_1.md)
- [inner](inner.md)
- [opening_preferences_reprobes_the_prefs_file](opening_preferences_reprobes_the_prefs_file.md) — Opening Preferences re-probes the file rather than trusting the boot
- [reopening_preferences_clears_the_reset_status](reopening_preferences_clears_the_reset_status.md) — Reopening the dialog is a fresh session for transient feedback: the
- [reporting_the_reset_outcome_does_not_mark_the_dialog_dirty](reporting_the_reset_outcome_does_not_mark_the_dialog_dirty.md) — The status line is feedback about one action, not an edit — reporting
- [resetting_a_healthy_prefs_file_moves_nothing_and_says_so](resetting_a_healthy_prefs_file_moves_nothing_and_says_so.md) — The guard is the whole reason the recovery action is safe to run beside
- [resetting_clears_a_stale_load_error_flag](resetting_clears_a_stale_load_error_flag.md) — The banner is state, and the action has to clear it — otherwise a user
- [restore](restore.md) — Put `path` back to `before` — content restored, or removed again when
- [saving_reprobes_a_file_that_broke_while_the_dialog_was_open](saving_reprobes_a_file_that_broke_while_the_dialog_was_open.md) — A file that breaks *while* Preferences is open has no other route to
- [seed_broken](seed_broken.md) — Put malformed JSON at the shared prefs path. `create_dir_all`
- [seed_broken](seed_broken_1.md) — Put malformed JSON at the shared prefs path. `create_dir_all`
- [serial](serial.md) — A poisoned lock is not a reason to fail every remaining test: the guard
