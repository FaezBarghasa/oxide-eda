---
okf_version: "0.2"
type: Function
title: saving_reprobes_a_file_that_broke_while_the_dialog_was_open
description: "A file that breaks *while* Preferences is open has no other route to"
resource: crates/oxide-app/tests/regression/preferences_prefs_recovery.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/preferences_prefs_recovery/saving_reprobes_a_file_that_broke_while_the_dialog_was_open
language: rust
---

# saving_reprobes_a_file_that_broke_while_the_dialog_was_open

A file that breaks *while* Preferences is open has no other route to

## Signature

```rust
fn saving_reprobes_a_file_that_broke_while_the_dialog_was_open()
```

## Decorators

- `test`

## Docstring

A file that breaks *while* Preferences is open has no other route to
the banner: `handle_preferences_open_requested` early-returns when the
dialog is already up, so without the probe at the end of the Save arm
every write is refused while the dialog reports success — the exact
invisible-failure state #602 exists to remove.
[test]

## Source
Lines 265–293 in `crates/oxide-app/tests/regression/preferences_prefs_recovery.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences_prefs_recovery](/crates/oxide-app/tests/regression/preferences_prefs_recovery.md) |
| calls | [serial](/crates/oxide-app/tests/regression/preferences_prefs_recovery/serial.md) |
| calls | [inner](/crates/oxide-app/tests/regression/preferences_prefs_recovery/inner.md) |
