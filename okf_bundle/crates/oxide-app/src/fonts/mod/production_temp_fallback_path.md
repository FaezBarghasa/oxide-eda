---
okf_version: "0.2"
type: Function
title: production_temp_fallback_path
description: "Fallback used only when `dirs::config_dir()` returns `None` (rare — a"
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod/production_temp_fallback_path
language: rust
---

# production_temp_fallback_path

Fallback used only when `dirs::config_dir()` returns `None` (rare — a

## Signature

```rust
fn production_temp_fallback_path() -> PathBuf
```

## Docstring

Fallback used only when `dirs::config_dir()` returns `None` (rare — a
Windows account with no `%APPDATA%`, a daemon/container with neither
`$XDG_CONFIG_HOME` nor `$HOME`). Issue #437's review flagged the naive
version of this fallback (a bare `<tmp>/oxide/prefs.json`) as
predictable and shared: two users on one host collide (the second gets
`EACCES` from `atomic_write`, visible only at `tracing::debug!`, or
silently reads the first user's prefs), and an attacker who pre-creates
`prefs.json.tmp` as a symlink gets `atomic_write`'s `File::create` to
write through it.

A later per-user `<tmp>/oxide-{user}` scheme fixed the sharing
problem but leaned on `USER`/`USERNAME`/`LOGNAME` being set — a
bare-uid container or a systemd unit with a scrubbed environment has
none of those, and that version `panic!`ed there, taking the whole
app down before the window ever appeared (#440 review: "fail loudly"
was meant to mean a visible log line, not a vanished process).

`tempfile::Builder::tempdir()` replaces that scheme outright: the
directory name is process-random (nothing to pre-guess for the
symlink attack above, no username needed) and it's created 0700 on
Unix — strictly safer than the username-keyed scheme even on the
happy path. If even creating a temp directory fails (the temp
filesystem itself is unwritable — every other option has already
failed too), this hands back a path anyway rather than panicking:
every write through it fails at `atomic_write`'s own
`tracing::debug!`, and the app simply runs the session on in-memory
defaults. Degrade, don't die.

## Source
Lines 260–295 in `crates/oxide-app/src/fonts/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fonts](/crates/oxide-app/src/fonts/mod.md) |
| called_by | [prefs_path](/crates/oxide-app/src/fonts/mod/prefs_path.md) |
