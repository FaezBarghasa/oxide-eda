---
okf_version: "0.2"
type: Function
title: inner
resource: crates/oxide-app/tests/regression/preferences_export_status.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/preferences_export_status/inner
language: rust
---

# inner

## Signature

```rust
fn inner(msg: PrefMsg) -> Message
```

## Source
Lines 19–21 in `crates/oxide-app/tests/regression/preferences_export_status.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences_export_status](/crates/oxide-app/tests/regression/preferences_export_status.md) |
| calls | [Inner](/crates/oxide-library-server/src/locks/Inner.md) |
| called_by | [keymap_export_failure_is_reported_and_names_the_cause](/crates/oxide-app/tests/regression/preferences_export_status/keymap_export_failure_is_reported_and_names_the_cause.md) |
| called_by | [keymap_export_success_names_the_written_path](/crates/oxide-app/tests/regression/preferences_export_status/keymap_export_success_names_the_written_path.md) |
| called_by | [reporting_an_export_outcome_does_not_mark_the_dialog_dirty](/crates/oxide-app/tests/regression/preferences_export_status/reporting_an_export_outcome_does_not_mark_the_dialog_dirty.md) |
| called_by | [theme_export_failure_is_reported_and_names_the_cause](/crates/oxide-app/tests/regression/preferences_export_status/theme_export_failure_is_reported_and_names_the_cause.md) |
| called_by | [theme_export_success_names_the_written_path](/crates/oxide-app/tests/regression/preferences_export_status/theme_export_success_names_the_written_path.md) |
