---
okf_version: "0.2"
type: Function
title: moving_aside_reports_a_failure_it_cannot_tell_apart_from_absence
description: "`Path::exists()` swallows every errno, so it reports `false` for a"
resource: crates/oxide-app/src/fonts/prefs_file.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/prefs_file/moving_aside_reports_a_failure_it_cannot_tell_apart_from_absence
language: rust
---

# moving_aside_reports_a_failure_it_cannot_tell_apart_from_absence

`Path::exists()` swallows every errno, so it reports `false` for a

## Signature

```rust
fn moving_aside_reports_a_failure_it_cannot_tell_apart_from_absence()
```

## Decorators

- `test`

## Docstring

`Path::exists()` swallows every errno, so it reports `false` for a
file it merely cannot stat — and the caller then tells the user
"there was no preferences file to move aside", clears the banner,
and leaves every write refused for the session. A path that cannot
be reached is a failure, never an absence. A regular file standing
in for a directory component is the portable way to force that:
the syscalls come back `ENOTDIR`, not `ENOENT`.
[test]

## Source
Lines 768–799 in `crates/oxide-app/src/fonts/prefs_file.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs_file](/crates/oxide-app/src/fonts/prefs_file.md) |
| calls | [temp_prefs](/crates/oxide-app/src/fonts/prefs_file/temp_prefs.md) |
| calls | [move_prefs_file_aside_at](/crates/oxide-app/src/fonts/prefs_file/move_prefs_file_aside_at.md) |
