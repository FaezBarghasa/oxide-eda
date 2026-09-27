---
okf_version: "0.2"
type: Function
title: refresh_prefs_load_error
description: "Re-read `prefs.json`'s health into the banner flag (#602)."
resource: crates/oxide-app/src/app/handlers/preferences/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/preferences/mod/refresh_prefs_load_error_1
language: rust
---

# refresh_prefs_load_error

Re-read `prefs.json`'s health into the banner flag (#602).

## Signature

```rust
fn refresh_prefs_load_error(&mut self)
```

## Docstring

Re-read `prefs.json`'s health into the banner flag (#602).

One cheap file read, so it can run on every Preferences open and
after every burst of preference writes rather than being trusted
from boot. That second call site is what catches a file broken
*while* the dialog is open: `handle_preferences_open_requested`
early-returns when the dialog is already up, so without a probe on
the Save path a user who hand-edits the file beside the running app
— the very workflow the banner's "repair it by hand" advice invites
— gets every write refused with the dialog still reporting success.

## Source
Lines 116–120 in `crates/oxide-app/src/app/handlers/preferences/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences](/crates/oxide-app/src/app/handlers/preferences/mod.md) |
| calls | [check_prefs_file](/crates/oxide-app/src/fonts/prefs_file/check_prefs_file.md) |
