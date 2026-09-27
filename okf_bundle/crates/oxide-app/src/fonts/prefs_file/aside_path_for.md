---
okf_version: "0.2"
type: Function
title: aside_path_for
description: First free sibling name to move a broken prefs file to.
resource: crates/oxide-app/src/fonts/prefs_file.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/prefs_file/aside_path_for
language: rust
---

# aside_path_for

First free sibling name to move a broken prefs file to.

## Signature

```rust
fn aside_path_for(path: &Path) -> Result<PathBuf, std::io::Error>
```

## Docstring

First free sibling name to move a broken prefs file to.

Appended to the WHOLE file name, so `prefs.json` moves to
`prefs.json.bak` rather than the `prefs.bak` that `set_extension`
would produce — and built through [`std::ffi::OsString`] so a
non-UTF-8 config directory survives instead of being mangled by a
lossy round-trip. Same reasoning as `keymap::profile::backup_path_for`.

Unlike that one, an existing `.bak` does not stop the search: it
ladders to `prefs.json.bak.2`, `.3`, and so on. The keymap backup can
stop, because there the `.bak` IS the original and the save that
follows overwrites in place. Here the whole point is to get the
broken file off `path`, and stopping would trap a user who has to
reset a second time.

A slot is free only when `symlink_metadata` answers `NotFound`. An
error that is anything else is not "occupied", it is "cannot tell",
and it aborts the search carrying that errno rather than laddering
past it. Both halves of that matter:

- Treating "cannot tell" as free is what `Path::exists()` does — it is
`metadata().is_ok()` and answers `false` for a name it merely cannot
stat, which would hand [`std::fs::rename`] a destination it then
silently replaces. POSIX `rename` overwrites, so the file that
vanished would be the user's earlier backup.
- Treating it as occupied is safe but dishonest. On a config directory
at mode 0000 every one of the [`MAX_ASIDE_SLOTS`] candidates fails
the same way, and the caller would report "all 100 .bak slots are
already taken" for what is really `Permission denied (os error 13)`
— a false cause, on the one screen whose job is to tell the user
what is actually wrong with their file.

`symlink_metadata` and not `metadata`, so a dangling symlink counts as
occupied: `rename` operates on the link itself, and `metadata` would
follow it to a missing target and call the slot free.

## Source
Lines 271–300 in `crates/oxide-app/src/fonts/prefs_file.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs_file](/crates/oxide-app/src/fonts/prefs_file.md) |
| called_by | [move_prefs_file_aside_at](/crates/oxide-app/src/fonts/prefs_file/move_prefs_file_aside_at.md) |
