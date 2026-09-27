---
okf_version: "0.2"
type: Module
title: preferences_export_status
description: "#533 Class C — a failed export write must reach the user."
resource: crates/oxide-app/tests/regression/preferences_export_status.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/preferences_export_status
language: rust
---

# preferences_export_status

#533 Class C — a failed export write must reach the user.

## Docstring

#533 Class C — a failed export write must reach the user.

Both Preferences exports (Appearance ▸ Export Theme, Keyboard
Shortcuts ▸ Export Profile) wrote to a file the user had just picked
in a save dialog and then discarded the `io::Result` with
`let _ = f.write(...).await`. A read-only volume, a full disk, or a
path that vanished between the pick and the write all produced the
exact same silent `Message::Noop` as a successful export, so the
user was left believing a file had been written that had not.

The `rfd::AsyncFileDialog` pick itself still needs a human — these
tests drive the completion messages the async task now emits, which
is where the reporting lives.

## Relationships

| Type | Target |
|------|--------|
| related | [inner](/crates/oxide-app/tests/regression/preferences_export_status/inner.md) |
| related | [theme_export_failure_is_reported_and_names_the_cause](/crates/oxide-app/tests/regression/preferences_export_status/theme_export_failure_is_reported_and_names_the_cause.md) |
| related | [theme_export_success_names_the_written_path](/crates/oxide-app/tests/regression/preferences_export_status/theme_export_success_names_the_written_path.md) |
| related | [reporting_an_export_outcome_does_not_mark_the_dialog_dirty](/crates/oxide-app/tests/regression/preferences_export_status/reporting_an_export_outcome_does_not_mark_the_dialog_dirty.md) |
| related | [keymap_export_failure_is_reported_and_names_the_cause](/crates/oxide-app/tests/regression/preferences_export_status/keymap_export_failure_is_reported_and_names_the_cause.md) |
| related | [keymap_export_success_names_the_written_path](/crates/oxide-app/tests/regression/preferences_export_status/keymap_export_success_names_the_written_path.md) |
| related | [opening_preferences_clears_a_stale_export_status](/crates/oxide-app/tests/regression/preferences_export_status/opening_preferences_clears_a_stale_export_status.md) |
