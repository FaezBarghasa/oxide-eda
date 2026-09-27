---
okf_version: "0.2"
type: Module
title: prefs_file
description: "The `prefs.json` file's health, its non-clobbering read-modify-write,"
resource: crates/oxide-app/src/fonts/prefs_file.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/prefs_file
language: rust
---

# prefs_file

The `prefs.json` file's health, its non-clobbering read-modify-write,

## Docstring

The `prefs.json` file's health, its non-clobbering read-modify-write,
and the one recovery action that gets a broken file out of the way
(#594, #602).

Every preference writer in this module tree is a read-modify-write:
load the whole file, set one key, write the whole file back. The
chain that used to do the "load" half discarded both the read error
and the parse error with `.ok()` and fell back to an empty JSON
object, which collapsed three different states into one: "the file
does not exist"
(a fresh install, where `{}` is right), "the file could not be read"
(a permission or hardware fault) and "the file did not parse" (a
truncated write, a hand-edit with a missing brace). In the last two
the user *has* preferences on disk, and starting from `{}` meant the
very next toggle of any single knob rewrote the file as that one key
alone — theme, dock layout, ERC severity overrides, the pin matrix,
filter presets and component classes all gone, with no error shown.

[`update_prefs_json`] refuses instead: on anything but genuine
absence it reports and returns, leaving the file byte-identical so
the user still has something to repair.

Refusing is only half an answer, though. Every *reader* in this module
tree swallows the same failure with `.ok()` and hands back its default,
so a broken file makes the whole UI look factory-reset while nothing
the user changes sticks — and the only report is a `tracing::error!`
in a dock panel they may never open. [`check_prefs_file`] is the
health probe the UI asks at boot and on every Preferences open, and
[`move_prefs_file_aside`] is the one in-app repair: rename the broken
file to a free `.bak` slot and drop a fresh `{}` in its place, so the
next [`update_prefs_json`] has a loadable file to build on and the
original is still on disk beside it (#602). The empty object rather
than an empty slot is deliberate — [`super::migrate_legacy_prefs`]
copies a legacy file forward on exactly `!canonical.exists()`, and
would otherwise undo the reset on the next launch.

## Relationships

| Type | Target |
|------|--------|
| related | [PrefsLoadError](/crates/oxide-app/src/fonts/prefs_file/PrefsLoadError.md) |
| related | [fmt](/crates/oxide-app/src/fonts/prefs_file/fmt.md) |
| related | [fmt](/crates/oxide-app/src/fonts/prefs_file/fmt.md) |
| related | [load_for_update](/crates/oxide-app/src/fonts/prefs_file/load_for_update.md) |
| related | [reported_failures](/crates/oxide-app/src/fonts/prefs_file/reported_failures.md) |
| related | [latch](/crates/oxide-app/src/fonts/prefs_file/latch.md) |
| related | [report_refusal](/crates/oxide-app/src/fonts/prefs_file/report_refusal.md) |
| related | [forget_refusal](/crates/oxide-app/src/fonts/prefs_file/forget_refusal.md) |
| related | [update_prefs_json](/crates/oxide-app/src/fonts/prefs_file/update_prefs_json.md) |
| related | [prefs_file_path](/crates/oxide-app/src/fonts/prefs_file/prefs_file_path.md) |
| related | [check_prefs_file_at](/crates/oxide-app/src/fonts/prefs_file/check_prefs_file_at.md) |
| related | [check_prefs_file](/crates/oxide-app/src/fonts/prefs_file/check_prefs_file.md) |
| related | [aside_path_for](/crates/oxide-app/src/fonts/prefs_file/aside_path_for.md) |
| related | [move_prefs_file_aside_at](/crates/oxide-app/src/fonts/prefs_file/move_prefs_file_aside_at.md) |
| related | [move_prefs_file_aside](/crates/oxide-app/src/fonts/prefs_file/move_prefs_file_aside.md) |
| related | [temp_prefs](/crates/oxide-app/src/fonts/prefs_file/temp_prefs.md) |
| related | [read_prefs_object](/crates/oxide-app/src/fonts/prefs_file/read_prefs_object.md) |
| related | [malformed_prefs_file_is_left_byte_identical](/crates/oxide-app/src/fonts/prefs_file/malformed_prefs_file_is_left_byte_identical.md) |
| related | [unreadable_prefs_file_is_left_alone](/crates/oxide-app/src/fonts/prefs_file/unreadable_prefs_file_is_left_alone.md) |
| related | [absent_prefs_file_is_created](/crates/oxide-app/src/fonts/prefs_file/absent_prefs_file_is_created.md) |
| related | [empty_prefs_file_is_treated_as_absent](/crates/oxide-app/src/fonts/prefs_file/empty_prefs_file_is_treated_as_absent.md) |
| related | [non_object_root_is_left_alone_and_does_not_panic](/crates/oxide-app/src/fonts/prefs_file/non_object_root_is_left_alone_and_does_not_panic.md) |
| related | [existing_keys_survive_an_unrelated_write](/crates/oxide-app/src/fonts/prefs_file/existing_keys_survive_an_unrelated_write.md) |
| related | [checking_an_absent_file_reports_it_as_healthy](/crates/oxide-app/src/fonts/prefs_file/checking_an_absent_file_reports_it_as_healthy.md) |
| related | [checking_an_empty_file_reports_it_as_healthy](/crates/oxide-app/src/fonts/prefs_file/checking_an_empty_file_reports_it_as_healthy.md) |
| related | [checking_a_valid_object_reports_it_as_healthy](/crates/oxide-app/src/fonts/prefs_file/checking_a_valid_object_reports_it_as_healthy.md) |
| related | [checking_a_malformed_file_reports_a_parse_error](/crates/oxide-app/src/fonts/prefs_file/checking_a_malformed_file_reports_a_parse_error.md) |
| related | [checking_a_non_object_root_reports_it](/crates/oxide-app/src/fonts/prefs_file/checking_a_non_object_root_reports_it.md) |
| related | [checking_a_file_never_creates_or_modifies_it](/crates/oxide-app/src/fonts/prefs_file/checking_a_file_never_creates_or_modifies_it.md) |
| related | [moving_aside_leaves_the_prefs_path_loadable_again](/crates/oxide-app/src/fonts/prefs_file/moving_aside_leaves_the_prefs_path_loadable_again.md) |
| related | [moving_aside_is_not_undone_by_the_legacy_prefs_migration](/crates/oxide-app/src/fonts/prefs_file/moving_aside_is_not_undone_by_the_legacy_prefs_migration.md) |
| related | [moving_aside_reports_a_failure_it_cannot_tell_apart_from_absence](/crates/oxide-app/src/fonts/prefs_file/moving_aside_reports_a_failure_it_cannot_tell_apart_from_absence.md) |
| related | [moving_aside_appends_bak_to_the_whole_file_name](/crates/oxide-app/src/fonts/prefs_file/moving_aside_appends_bak_to_the_whole_file_name.md) |
| related | [moving_aside_never_overwrites_an_existing_backup](/crates/oxide-app/src/fonts/prefs_file/moving_aside_never_overwrites_an_existing_backup.md) |
| related | [moving_aside_is_a_no_op_when_there_is_no_file](/crates/oxide-app/src/fonts/prefs_file/moving_aside_is_a_no_op_when_there_is_no_file.md) |
| related | [writes_resume_after_the_broken_file_is_moved_aside](/crates/oxide-app/src/fonts/prefs_file/writes_resume_after_the_broken_file_is_moved_aside.md) |
