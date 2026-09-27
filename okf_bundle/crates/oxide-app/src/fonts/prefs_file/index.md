# prefs_file

## Classs

- [PrefsLoadError](PrefsLoadError.md) — Why an existing `prefs.json` could not be loaded for an in-place

## Functions

- [absent_prefs_file_is_created](absent_prefs_file_is_created.md) — The fresh-install path: absence is the one state that legitimately
- [aside_path_for](aside_path_for.md) — First free sibling name to move a broken prefs file to.
- [check_prefs_file](check_prefs_file.md) — [`check_prefs_file_at`] against the resolved user prefs path,
- [check_prefs_file_at](check_prefs_file_at.md) — Is the `prefs.json` at `path` in a state a write can build on?
- [checking_a_file_never_creates_or_modifies_it](checking_a_file_never_creates_or_modifies_it.md) — The probe runs on every Preferences open, so it must be inert: it
- [checking_a_malformed_file_reports_a_parse_error](checking_a_malformed_file_reports_a_parse_error.md) — [test]
- [checking_a_non_object_root_reports_it](checking_a_non_object_root_reports_it.md) — [test]
- [checking_a_valid_object_reports_it_as_healthy](checking_a_valid_object_reports_it_as_healthy.md) — [test]
- [checking_an_absent_file_reports_it_as_healthy](checking_an_absent_file_reports_it_as_healthy.md) — Absence is the fresh-install path the writer starts from, so the
- [checking_an_empty_file_reports_it_as_healthy](checking_an_empty_file_reports_it_as_healthy.md) — Same rule as [`empty_prefs_file_is_treated_as_absent`]: a writer
- [empty_prefs_file_is_treated_as_absent](empty_prefs_file_is_treated_as_absent.md) — A zero-byte file carries no user data to protect and is what an
- [existing_keys_survive_an_unrelated_write](existing_keys_survive_an_unrelated_write.md) — The base property the whole module exists for: writing one key
- [fmt](fmt.md)
- [fmt](fmt_1.md)
- [forget_refusal](forget_refusal.md) — Forget a path that loaded cleanly, so a file that is repaired and
- [latch](latch.md) — Recover a poisoned latch instead of panicking on it: the set is a
- [load_for_update](load_for_update.md) — Load `prefs.json` as the object it is supposed to be, or say why not.
- [malformed_prefs_file_is_left_byte_identical](malformed_prefs_file_is_left_byte_identical.md) — The #594 regression: a truncated file (killed mid-write, or
- [move_prefs_file_aside](move_prefs_file_aside.md) — [`move_prefs_file_aside_at`] against the resolved user prefs path.
- [move_prefs_file_aside_at](move_prefs_file_aside_at.md) — Move the `prefs.json` at `path` aside so the next write starts a
- [moving_aside_appends_bak_to_the_whole_file_name](moving_aside_appends_bak_to_the_whole_file_name.md) — `set_extension` would turn `prefs.json` into `prefs.bak` and lose
- [moving_aside_is_a_no_op_when_there_is_no_file](moving_aside_is_a_no_op_when_there_is_no_file.md) — [test]
- [moving_aside_is_not_undone_by_the_legacy_prefs_migration](moving_aside_is_not_undone_by_the_legacy_prefs_migration.md) — Leaving the prefs path absent handed the reset straight back to
- [moving_aside_leaves_the_prefs_path_loadable_again](moving_aside_leaves_the_prefs_path_loadable_again.md) — The invariant is not "the path is empty" — that was the shape, and
- [moving_aside_never_overwrites_an_existing_backup](moving_aside_never_overwrites_an_existing_backup.md) — The keymap backup returns `Ok(None)` when a `.bak` exists, which
- [moving_aside_reports_a_failure_it_cannot_tell_apart_from_absence](moving_aside_reports_a_failure_it_cannot_tell_apart_from_absence.md) — `Path::exists()` swallows every errno, so it reports `false` for a
- [non_object_root_is_left_alone_and_does_not_panic](non_object_root_is_left_alone_and_does_not_panic.md) — A non-object root used to reach the `library_browser_searches`
- [prefs_file_path](prefs_file_path.md) — The resolved `prefs.json` path, for UI that has to *name* the file.
- [read_prefs_object](read_prefs_object.md)
- [report_refusal](report_refusal.md) — Report a refused write once, at `Error` level — the default filter
- [reported_failures](reported_failures.md) — Paths currently in the refused state, so the report below fires on
- [temp_prefs](temp_prefs.md) — A tempdir plus the `prefs.json` path inside it. Deliberately not
- [unreadable_prefs_file_is_left_alone](unreadable_prefs_file_is_left_alone.md) — A directory standing where the file should be is the portable
- [update_prefs_json](update_prefs_json.md) — Update one key of `prefs.json` at `path` without clobbering the
- [writes_resume_after_the_broken_file_is_moved_aside](writes_resume_after_the_broken_file_is_moved_aside.md) — The end-to-end shape of #602: a refused write, the recovery, and
