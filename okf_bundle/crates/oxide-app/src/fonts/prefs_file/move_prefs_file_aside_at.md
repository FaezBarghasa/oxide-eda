---
okf_version: "0.2"
type: Function
title: move_prefs_file_aside_at
description: "Move the `prefs.json` at `path` aside so the next write starts a"
resource: crates/oxide-app/src/fonts/prefs_file.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/prefs_file/move_prefs_file_aside_at
language: rust
---

# move_prefs_file_aside_at

Move the `prefs.json` at `path` aside so the next write starts a

## Signature

```rust
pub fn move_prefs_file_aside_at(path: &Path) -> Result<Option<PathBuf>, std::io::Error>
```

## Visibility

- `pub`

## Docstring

Move the `prefs.json` at `path` aside so the next write starts a
fresh one, keeping the old file on disk (#602).

A rename and not a copy: [`update_prefs_json`] refuses on anything but
genuine absence, so the broken file has to stop being at `path` for
writes to resume at all. Nothing is deleted — the bytes the user may
still want to repair by hand land at the returned path.

`path` is then re-seeded with `{}` rather than left absent, so
`super::migrate_legacy_prefs` cannot copy a stale legacy file back over
the fresh start on the next launch. `{}` is indistinguishable from
absence to every reader here.

`Ok(None)` means there was no file to move; that is not a failure, and
writes were never blocked in the first place. `Err` means the file is
still exactly where it was, so the caller must keep reporting it
broken rather than telling the user they have a fresh start.

# Precondition

Only call this for a path [`check_prefs_file_at`] has just reported as
unusable. The two deliberately answer different questions and can
disagree: `check_prefs_file_at` reads through a symlink, so a
`prefs.json` pointing at a detached volume reads as `NotFound` and is
called healthy, while `rename` here operates on the link itself and
would move it aside and seed `{}` over it — orphaning the real file
behind the `.bak` link once the volume returns. The single caller
(`PrefMsg::ResetPrefsFile`) re-checks immediately before calling, which
is what keeps that unreachable; keep that guard if you add a caller.

## Source
Lines 331–383 in `crates/oxide-app/src/fonts/prefs_file.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs_file](/crates/oxide-app/src/fonts/prefs_file.md) |
| calls | [aside_path_for](/crates/oxide-app/src/fonts/prefs_file/aside_path_for.md) |
| calls | [atomic_write](/crates/oxide-app/src/app/dispatch/library/recovery/atomic_write.md) |
| calls | [forget_refusal](/crates/oxide-app/src/fonts/prefs_file/forget_refusal.md) |
| called_by | [move_prefs_file_aside](/crates/oxide-app/src/fonts/prefs_file/move_prefs_file_aside.md) |
| called_by | [moving_aside_appends_bak_to_the_whole_file_name](/crates/oxide-app/src/fonts/prefs_file/moving_aside_appends_bak_to_the_whole_file_name.md) |
| called_by | [moving_aside_is_a_no_op_when_there_is_no_file](/crates/oxide-app/src/fonts/prefs_file/moving_aside_is_a_no_op_when_there_is_no_file.md) |
| called_by | [moving_aside_is_not_undone_by_the_legacy_prefs_migration](/crates/oxide-app/src/fonts/prefs_file/moving_aside_is_not_undone_by_the_legacy_prefs_migration.md) |
| called_by | [moving_aside_leaves_the_prefs_path_loadable_again](/crates/oxide-app/src/fonts/prefs_file/moving_aside_leaves_the_prefs_path_loadable_again.md) |
| called_by | [moving_aside_never_overwrites_an_existing_backup](/crates/oxide-app/src/fonts/prefs_file/moving_aside_never_overwrites_an_existing_backup.md) |
| called_by | [moving_aside_reports_a_failure_it_cannot_tell_apart_from_absence](/crates/oxide-app/src/fonts/prefs_file/moving_aside_reports_a_failure_it_cannot_tell_apart_from_absence.md) |
| called_by | [writes_resume_after_the_broken_file_is_moved_aside](/crates/oxide-app/src/fonts/prefs_file/writes_resume_after_the_broken_file_is_moved_aside.md) |
